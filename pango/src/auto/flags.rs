// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    pub struct FontMask: u32 {
        const FAMILY = 1;
        const STYLE = 2;
        const VARIANT = 4;
        const WEIGHT = 8;
        const STRETCH = 16;
        const SIZE = 32;
        const GRAVITY = 64;
        const VARIATIONS = 128;
    }
}

impl fmt::Display for FontMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for FontMask {
    type GlibType = ffi::PangoFontMask;

    fn to_glib(&self) -> ffi::PangoFontMask {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoFontMask> for FontMask {
    unsafe fn from_glib(value: ffi::PangoFontMask) -> FontMask {
        FontMask::from_bits_truncate(value)
    }
}

impl StaticType for FontMask {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_font_mask_get_type()) }
    }
}

impl glib::value::ValueType for FontMask {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FontMask {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FontMask {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<FontMask>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
bitflags! {
    pub struct ShapeFlags: u32 {
        const NONE = 0;
        const ROUND_POSITIONS = 1;
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl fmt::Display for ShapeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl ToGlib for ShapeFlags {
    type GlibType = ffi::PangoShapeFlags;

    fn to_glib(&self) -> ffi::PangoShapeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoShapeFlags> for ShapeFlags {
    unsafe fn from_glib(value: ffi::PangoShapeFlags) -> ShapeFlags {
        ShapeFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl StaticType for ShapeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_shape_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl glib::value::ValueType for ShapeFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
unsafe impl<'a> FromValue<'a> for ShapeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl ToValue for ShapeFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<ShapeFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
bitflags! {
    pub struct ShowFlags: u32 {
        const NONE = 0;
        const SPACES = 1;
        const LINE_BREAKS = 2;
        const IGNORABLES = 4;
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl fmt::Display for ShowFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl ToGlib for ShowFlags {
    type GlibType = ffi::PangoShowFlags;

    fn to_glib(&self) -> ffi::PangoShowFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoShowFlags> for ShowFlags {
    unsafe fn from_glib(value: ffi::PangoShowFlags) -> ShowFlags {
        ShowFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl StaticType for ShowFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_show_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl glib::value::ValueType for ShowFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
unsafe impl<'a> FromValue<'a> for ShowFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
impl ToValue for ShowFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<ShowFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
