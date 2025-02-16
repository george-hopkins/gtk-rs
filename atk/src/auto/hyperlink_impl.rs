// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Hyperlink;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct HyperlinkImpl(Interface<ffi::AtkHyperlinkImpl, ffi::AtkHyperlinkImplIface>);

    match fn {
        type_ => || ffi::atk_hyperlink_impl_get_type(),
    }
}

pub const NONE_HYPERLINK_IMPL: Option<&HyperlinkImpl> = None;

pub trait HyperlinkImplExt: 'static {
    #[doc(alias = "atk_hyperlink_impl_get_hyperlink")]
    fn hyperlink(&self) -> Option<Hyperlink>;
}

impl<O: IsA<HyperlinkImpl>> HyperlinkImplExt for O {
    fn hyperlink(&self) -> Option<Hyperlink> {
        unsafe {
            from_glib_full(ffi::atk_hyperlink_impl_get_hyperlink(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for HyperlinkImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HyperlinkImpl")
    }
}
