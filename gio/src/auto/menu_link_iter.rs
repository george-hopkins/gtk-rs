// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::MenuModel;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct MenuLinkIter(Object<ffi::GMenuLinkIter, ffi::GMenuLinkIterClass>);

    match fn {
        type_ => || ffi::g_menu_link_iter_get_type(),
    }
}

pub const NONE_MENU_LINK_ITER: Option<&MenuLinkIter> = None;

pub trait MenuLinkIterExt: 'static {
    #[doc(alias = "g_menu_link_iter_get_next")]
    fn next(&self) -> Option<(glib::GString, MenuModel)>;
}

impl<O: IsA<MenuLinkIter>> MenuLinkIterExt for O {
    fn next(&self) -> Option<(glib::GString, MenuModel)> {
        unsafe {
            let mut out_link = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::g_menu_link_iter_get_next(
                self.as_ref().to_glib_none().0,
                &mut out_link,
                &mut value,
            ));
            if ret {
                Some((from_glib_none(out_link), from_glib_full(value)))
            } else {
                None
            }
        }
    }
}

impl fmt::Display for MenuLinkIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MenuLinkIter")
    }
}
