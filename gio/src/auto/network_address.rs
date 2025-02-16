// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SocketConnectable;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct NetworkAddress(Object<ffi::GNetworkAddress, ffi::GNetworkAddressClass>) @implements SocketConnectable;

    match fn {
        type_ => || ffi::g_network_address_get_type(),
    }
}

impl NetworkAddress {
    #[doc(alias = "g_network_address_new")]
    pub fn new(hostname: &str, port: u16) -> NetworkAddress {
        unsafe { from_glib_full(ffi::g_network_address_new(hostname.to_glib_none().0, port)) }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    #[doc(alias = "g_network_address_new_loopback")]
    pub fn new_loopback(port: u16) -> NetworkAddress {
        unsafe { from_glib_full(ffi::g_network_address_new_loopback(port)) }
    }

    #[doc(alias = "g_network_address_parse")]
    pub fn parse(host_and_port: &str, default_port: u16) -> Result<NetworkAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_network_address_parse(
                host_and_port.to_glib_none().0,
                default_port,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_network_address_parse_uri")]
    pub fn parse_uri(uri: &str, default_port: u16) -> Result<NetworkAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_network_address_parse_uri(uri.to_glib_none().0, default_port, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl Send for NetworkAddress {}
unsafe impl Sync for NetworkAddress {}

pub const NONE_NETWORK_ADDRESS: Option<&NetworkAddress> = None;

pub trait NetworkAddressExt: 'static {
    #[doc(alias = "g_network_address_get_hostname")]
    fn hostname(&self) -> glib::GString;

    #[doc(alias = "g_network_address_get_port")]
    fn port(&self) -> u16;

    #[doc(alias = "g_network_address_get_scheme")]
    fn scheme(&self) -> Option<glib::GString>;
}

impl<O: IsA<NetworkAddress>> NetworkAddressExt for O {
    fn hostname(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_network_address_get_hostname(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn port(&self) -> u16 {
        unsafe { ffi::g_network_address_get_port(self.as_ref().to_glib_none().0) }
    }

    fn scheme(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_network_address_get_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for NetworkAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NetworkAddress")
    }
}
