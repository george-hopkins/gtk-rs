// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct X11Keymap(Object<ffi::GdkX11Keymap, ffi::GdkX11KeymapClass>) @extends gdk::Keymap;

    match fn {
        type_ => || ffi::gdk_x11_keymap_get_type(),
    }
}

impl X11Keymap {
    #[doc(alias = "gdk_x11_keymap_get_group_for_state")]
    pub fn group_for_state(&self, state: u32) -> i32 {
        unsafe { ffi::gdk_x11_keymap_get_group_for_state(self.to_glib_none().0, state) }
    }

    #[doc(alias = "gdk_x11_keymap_key_is_modifier")]
    pub fn key_is_modifier(&self, keycode: u32) -> bool {
        unsafe {
            from_glib(ffi::gdk_x11_keymap_key_is_modifier(
                self.to_glib_none().0,
                keycode,
            ))
        }
    }
}

impl fmt::Display for X11Keymap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11Keymap")
    }
}
