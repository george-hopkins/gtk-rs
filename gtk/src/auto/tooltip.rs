// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::IconSize;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Tooltip(Object<ffi::GtkTooltip>);

    match fn {
        type_ => || ffi::gtk_tooltip_get_type(),
    }
}

impl Tooltip {
    #[doc(alias = "gtk_tooltip_set_custom")]
    pub fn set_custom<P: IsA<Widget>>(&self, custom_widget: Option<&P>) {
        unsafe {
            ffi::gtk_tooltip_set_custom(
                self.to_glib_none().0,
                custom_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tooltip_set_icon")]
    pub fn set_icon(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_tooltip_set_icon(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tooltip_set_icon_from_gicon")]
    pub fn set_icon_from_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>, size: IconSize) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_gicon(
                self.to_glib_none().0,
                gicon.map(|p| p.as_ref()).to_glib_none().0,
                size.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tooltip_set_icon_from_icon_name")]
    pub fn set_icon_from_icon_name(&self, icon_name: Option<&str>, size: IconSize) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
                size.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tooltip_set_markup")]
    pub fn set_markup(&self, markup: Option<&str>) {
        unsafe {
            ffi::gtk_tooltip_set_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tooltip_set_text")]
    pub fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::gtk_tooltip_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tooltip_set_tip_area")]
    pub fn set_tip_area(&self, rect: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_tooltip_set_tip_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tooltip_trigger_tooltip_query")]
    pub fn trigger_tooltip_query(display: &gdk::Display) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tooltip_trigger_tooltip_query(display.to_glib_none().0);
        }
    }
}

impl fmt::Display for Tooltip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Tooltip")
    }
}
