// Code generated by machine generator; DO NOT EDIT.

//! Utility for rfc3162 packet.
//!
//! This module handles the packet according to the following definition:
//! ```text
//! //! # -*- text -*-
//! # Copyright (C) 2020 The FreeRADIUS Server project and contributors
//! # This work is licensed under CC-BY version 4.0 https://creativecommons.org/licenses/by/4.0
//! # Version $Id$
//! #
//! #	Attributes and values defined in RFC 3162.
//! #	http://www.ietf.org/rfc/rfc3162.txt
//! #
//! #	$Id$
//! #
//! ATTRIBUTE	NAS-IPV6-Address			95	ipv6addr
//! ATTRIBUTE	Framed-Interface-Id			96	ifid
//! ATTRIBUTE	Framed-IPV6-Prefix			97	ipv6prefix
//! ATTRIBUTE	Login-IPV6-Host				98	ipv6addr
//! ATTRIBUTE	Framed-IPV6-Route			99	string
//! ATTRIBUTE	Framed-IPV6-Pool			100	string
//! ```

use std::net::Ipv6Addr;

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

pub const NAS_IPV6_ADDRESS_TYPE: AVPType = 95;
/// Delete all of `nas_ipv6_address` values from a packet.
pub fn delete_nas_ipv6_address(packet: &mut Packet) {
    packet.delete(NAS_IPV6_ADDRESS_TYPE);
}
/// Add `nas_ipv6_address` ipv6addr value to a packet.
pub fn add_nas_ipv6_address(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(NAS_IPV6_ADDRESS_TYPE, value));
}
/// Lookup a `nas_ipv6_address` ipv6addr value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `nas_ipv6_address`, it returns `None`.
pub fn lookup_nas_ipv6_address(packet: &Packet) -> Option<Result<Ipv6Addr, AVPError>> {
    packet
        .lookup(NAS_IPV6_ADDRESS_TYPE)
        .map(|v| v.encode_ipv6())
}
/// Lookup all of the `nas_ipv6_address` ipv6addr value from a packet.
pub fn lookup_all_nas_ipv6_address(packet: &Packet) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(NAS_IPV6_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const FRAMED_INTERFACE_ID_TYPE: AVPType = 96;
/// Delete all of `framed_interface_id` values from a packet.
pub fn delete_framed_interface_id(packet: &mut Packet) {
    packet.delete(FRAMED_INTERFACE_ID_TYPE);
}
/// Add `framed_interface_id` fixed-length octets value to a packet.
pub fn add_framed_interface_id(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    if value.len() != 8 {
        return Err(AVPError::InvalidAttributeLengthError(
            "8 bytes".to_owned(),
            value.len(),
        ));
    }
    packet.add(AVP::from_bytes(FRAMED_INTERFACE_ID_TYPE, value));
    Ok(())
}
/// Lookup a `framed_interface_id` fixed-length octets value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `framed_interface_id`, it returns `None`.
pub fn lookup_framed_interface_id(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(FRAMED_INTERFACE_ID_TYPE)
        .map(|v| v.encode_bytes())
}
/// Lookup all of the `framed_interface_id` fixed-length octets value from a packet.
pub fn lookup_all_framed_interface_id(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(FRAMED_INTERFACE_ID_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub const FRAMED_IPV6_PREFIX_TYPE: AVPType = 97;
/// Delete all of `framed_ipv6_prefix` values from a packet.
pub fn delete_framed_ipv6_prefix(packet: &mut Packet) {
    packet.delete(FRAMED_IPV6_PREFIX_TYPE);
}
/// Add `framed_ipv6_prefix` ipv6 prefix value to a packet.
pub fn add_framed_ipv6_prefix(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    packet.add(AVP::from_ipv6_prefix(FRAMED_IPV6_PREFIX_TYPE, value)?);
    Ok(())
}
/// Lookup a `framed_ipv6_prefix` ipv6 prefix value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `framed_ipv6_prefix`, it returns `None`.
pub fn lookup_framed_ipv6_prefix(packet: &Packet) -> Option<Result<Vec<u8>, AVPError>> {
    packet
        .lookup(FRAMED_IPV6_PREFIX_TYPE)
        .map(|v| v.encode_ipv6_prefix())
}
/// Lookup all of the `framed_ipv6_prefix` ipv6 prefix value from a packet.
pub fn lookup_all_framed_ipv6_prefix(packet: &Packet) -> Result<Vec<Vec<u8>>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(FRAMED_IPV6_PREFIX_TYPE) {
        vec.push(avp.encode_ipv6_prefix()?)
    }
    Ok(vec)
}

pub const LOGIN_IPV6_HOST_TYPE: AVPType = 98;
/// Delete all of `login_ipv6_host` values from a packet.
pub fn delete_login_ipv6_host(packet: &mut Packet) {
    packet.delete(LOGIN_IPV6_HOST_TYPE);
}
/// Add `login_ipv6_host` ipv6addr value to a packet.
pub fn add_login_ipv6_host(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(LOGIN_IPV6_HOST_TYPE, value));
}
/// Lookup a `login_ipv6_host` ipv6addr value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `login_ipv6_host`, it returns `None`.
pub fn lookup_login_ipv6_host(packet: &Packet) -> Option<Result<Ipv6Addr, AVPError>> {
    packet.lookup(LOGIN_IPV6_HOST_TYPE).map(|v| v.encode_ipv6())
}
/// Lookup all of the `login_ipv6_host` ipv6addr value from a packet.
pub fn lookup_all_login_ipv6_host(packet: &Packet) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(LOGIN_IPV6_HOST_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const FRAMED_IPV6_ROUTE_TYPE: AVPType = 99;
/// Delete all of `framed_ipv6_route` values from a packet.
pub fn delete_framed_ipv6_route(packet: &mut Packet) {
    packet.delete(FRAMED_IPV6_ROUTE_TYPE);
}
/// Add `framed_ipv6_route` string value to a packet.
pub fn add_framed_ipv6_route(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(FRAMED_IPV6_ROUTE_TYPE, value));
}
/// Lookup a `framed_ipv6_route` string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `framed_ipv6_route`, it returns `None`.
pub fn lookup_framed_ipv6_route(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(FRAMED_IPV6_ROUTE_TYPE)
        .map(|v| v.encode_string())
}
/// Lookup all of the `framed_ipv6_route` string value from a packet.
pub fn lookup_all_framed_ipv6_route(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(FRAMED_IPV6_ROUTE_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const FRAMED_IPV6_POOL_TYPE: AVPType = 100;
/// Delete all of `framed_ipv6_pool` values from a packet.
pub fn delete_framed_ipv6_pool(packet: &mut Packet) {
    packet.delete(FRAMED_IPV6_POOL_TYPE);
}
/// Add `framed_ipv6_pool` string value to a packet.
pub fn add_framed_ipv6_pool(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(FRAMED_IPV6_POOL_TYPE, value));
}
/// Lookup a `framed_ipv6_pool` string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `framed_ipv6_pool`, it returns `None`.
pub fn lookup_framed_ipv6_pool(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(FRAMED_IPV6_POOL_TYPE)
        .map(|v| v.encode_string())
}
/// Lookup all of the `framed_ipv6_pool` string value from a packet.
pub fn lookup_all_framed_ipv6_pool(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(FRAMED_IPV6_POOL_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}