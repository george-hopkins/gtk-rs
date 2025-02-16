// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FontFace;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    pub struct FontFamily(Object<ffi::PangoFontFamily, ffi::PangoFontFamilyClass>);

    match fn {
        type_ => || ffi::pango_font_family_get_type(),
    }
}

pub const NONE_FONT_FAMILY: Option<&FontFamily> = None;

pub trait FontFamilyExt: 'static {
    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_font_family_get_face")]
    fn face(&self, name: Option<&str>) -> Option<FontFace>;

    #[doc(alias = "pango_font_family_get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "pango_font_family_is_monospace")]
    fn is_monospace(&self) -> bool;

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_font_family_is_variable")]
    fn is_variable(&self) -> bool;

    #[doc(alias = "pango_font_family_list_faces")]
    fn list_faces(&self) -> Vec<FontFace>;
}

impl<O: IsA<FontFamily>> FontFamilyExt for O {
    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    fn face(&self, name: Option<&str>) -> Option<FontFace> {
        unsafe {
            from_glib_none(ffi::pango_font_family_get_face(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::pango_font_family_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_monospace(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_family_is_monospace(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    fn is_variable(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_family_is_variable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_faces(&self) -> Vec<FontFace> {
        unsafe {
            let mut faces = ptr::null_mut();
            let mut n_faces = mem::MaybeUninit::uninit();
            ffi::pango_font_family_list_faces(
                self.as_ref().to_glib_none().0,
                &mut faces,
                n_faces.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_container_num(faces, n_faces.assume_init() as usize)
        }
    }
}

impl fmt::Display for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FontFamily")
    }
}
