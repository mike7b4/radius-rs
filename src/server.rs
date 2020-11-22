use std::borrow::Borrow;
use std::collections::HashSet;
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use tokio::net::UdpSocket;

use crate::packet::Packet;
use crate::request::Request;
use crate::request_handler::RequestHandler;
use crate::secret_provider::SecretProvider;

pub struct Server<T: 'static + RequestHandler, U: 'static + SecretProvider> {
    address: String,
    buf_size: u8,
    skip_authenticity_validation: bool,
    request_handler: &'static T,
    secret_provider: &'static U,
}

impl<T: RequestHandler, U: SecretProvider> Server<T, U> {
    pub fn new(host: &str, port: u16, buf_size: u8, skip_authenticity_validation: bool, request_handler: &'static T, secret_provider: &'static U) -> Self {
        Self {
            address: format!("{}:{}", host, port),
            buf_size,
            skip_authenticity_validation,
            request_handler,
            secret_provider,
        }
    }

    pub async fn run(&'static self) -> Result<(), io::Error> {
        let mut buf = vec![0, self.buf_size];

        let conn_arc = Arc::new(UdpSocket::bind(&self.address).await?);
        let undergoing_requests_lock_arc = Arc::new(RwLock::new(HashSet::new()));

        loop {
            let conn = conn_arc.clone();

            let (size, remote_addr) = conn.recv_from(&mut buf).await?;
            let request_data = buf[..size].to_vec();

            let local_addr = match conn.local_addr() {
                Ok(addr) => addr,
                Err(e) => {
                    error!("failed to get a local address from from a connection; {}", e);
                    continue;
                }
            };

            let undergoing_requests_lock = undergoing_requests_lock_arc.clone();

            tokio::spawn(async move {
                Self::process_request(
                    conn,
                    &request_data,
                    local_addr,
                    remote_addr,
                    undergoing_requests_lock,
                    self.request_handler,
                    self.secret_provider,
                    self.skip_authenticity_validation,
                ).await;
            });
        }
    }

    async fn process_request(
        conn: Arc<UdpSocket>,
        request_data: &Vec<u8>,
        local_addr: SocketAddr,
        remote_addr: SocketAddr,
        undergoing_requests_lock: Arc<RwLock<HashSet<RequestKey>>>,
        request_handler: &T,
        secret_provider: &U,
        skip_authenticity_validation: bool,
    ) {
        let secret: Vec<u8> = match secret_provider.fetch_secret(remote_addr) {
            Ok(secret) => secret,
            Err(e) => {
                error!("failed to fetch secret binary vector from the secret provider; {}", e);
                return;
            }
        };
        if secret.len() <= 0 {
            error!("empty secret returned from secret source; empty secret is prohibited");
            return;
        }

        if !skip_authenticity_validation && !Packet::is_authentic_request(request_data, &secret) {
            info!("packet validation failed; bad secret");
            return;
        }

        let packet = match Packet::parse(request_data, &secret) {
            Ok(packet) => packet,
            Err(e) => {
                error!("failed to parse given request data to pack into the RADIUS packet; {}", e);
                debug!("failed request data => {:?}", request_data);
                return;
            }
        };

        let key = RequestKey {
            ip: remote_addr.to_string(),
            identifier: packet.get_identifier(),
        };
        let key_for_remove = key.clone();

        {
            let mut undergoing_requests = undergoing_requests_lock.write().unwrap();
            if undergoing_requests.contains(&key) {
                return;
            }
            undergoing_requests.insert(key);
        }

        request_handler.handle_radius_request(conn.borrow(), &Request::new(local_addr, remote_addr, packet));

        let mut undergoing_requests = undergoing_requests_lock.write().unwrap();
        undergoing_requests.remove(&key_for_remove);
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct RequestKey {
    ip: String,
    identifier: u8,
}
