// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SocketConnectable;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct NetworkService(Object<ffi::GNetworkService, ffi::GNetworkServiceClass>) @implements SocketConnectable;

    match fn {
        type_ => || ffi::g_network_service_get_type(),
    }
}

impl NetworkService {
    #[doc(alias = "g_network_service_new")]
    pub fn new(service: &str, protocol: &str, domain: &str) -> NetworkService {
        unsafe {
            from_glib_full(ffi::g_network_service_new(
                service.to_glib_none().0,
                protocol.to_glib_none().0,
                domain.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_NETWORK_SERVICE: Option<&NetworkService> = None;

pub trait NetworkServiceExt: 'static {
    #[doc(alias = "g_network_service_get_domain")]
    fn domain(&self) -> glib::GString;

    #[doc(alias = "g_network_service_get_protocol")]
    fn protocol(&self) -> glib::GString;

    #[doc(alias = "g_network_service_get_scheme")]
    fn scheme(&self) -> glib::GString;

    #[doc(alias = "g_network_service_get_service")]
    fn service(&self) -> glib::GString;

    #[doc(alias = "g_network_service_set_scheme")]
    fn set_scheme(&self, scheme: &str);

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NetworkService>> NetworkServiceExt for O {
    fn domain(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_network_service_get_domain(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn protocol(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_network_service_get_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn scheme(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_network_service_get_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn service(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_network_service_get_service(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_scheme(&self, scheme: &str) {
        unsafe {
            ffi::g_network_service_set_scheme(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scheme_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GNetworkService,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NetworkService>,
        {
            let f: &F = &*(f as *const F);
            f(&NetworkService::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scheme\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scheme_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for NetworkService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NetworkService")
    }
}
