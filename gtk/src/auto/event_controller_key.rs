// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::IMContext;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct EventControllerKey(Object<ffi::GtkEventControllerKey, ffi::GtkEventControllerKeyClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_key_get_type(),
    }
}

impl EventControllerKey {
    #[doc(alias = "gtk_event_controller_key_new")]
    pub fn new<P: IsA<Widget>>(widget: &P) -> EventControllerKey {
        skip_assert_initialized!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_key_new(
                widget.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_event_controller_key_forward")]
    pub fn forward<P: IsA<Widget>>(&self, widget: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_key_forward(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_key_get_group")]
    pub fn group(&self) -> u32 {
        unsafe { ffi::gtk_event_controller_key_get_group(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_event_controller_key_get_im_context")]
    pub fn im_context(&self) -> Option<IMContext> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_key_get_im_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_key_set_im_context")]
    pub fn set_im_context<P: IsA<IMContext>>(&self, im_context: &P) {
        unsafe {
            ffi::gtk_event_controller_key_set_im_context(
                self.to_glib_none().0,
                im_context.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn connect_focus_in<F: Fn(&EventControllerKey) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn focus_in_trampoline<F: Fn(&EventControllerKey) + 'static>(
            this: *mut ffi::GtkEventControllerKey,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-in\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    focus_in_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_focus_out<F: Fn(&EventControllerKey) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn focus_out_trampoline<F: Fn(&EventControllerKey) + 'static>(
            this: *mut ffi::GtkEventControllerKey,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-out\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    focus_out_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_im_update<F: Fn(&EventControllerKey) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn im_update_trampoline<F: Fn(&EventControllerKey) + 'static>(
            this: *mut ffi::GtkEventControllerKey,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"im-update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    im_update_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    pub fn connect_key_pressed<
        F: Fn(&EventControllerKey, u32, u32, gdk::ModifierType) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn key_pressed_trampoline<
            F: Fn(&EventControllerKey, u32, u32, gdk::ModifierType) -> bool + 'static,
        >(
            this: *mut ffi::GtkEventControllerKey,
            keyval: libc::c_uint,
            keycode: libc::c_uint,
            state: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), keyval, keycode, from_glib(state)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"key-pressed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    key_pressed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    pub fn connect_key_released<
        F: Fn(&EventControllerKey, u32, u32, gdk::ModifierType) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn key_released_trampoline<
            F: Fn(&EventControllerKey, u32, u32, gdk::ModifierType) + 'static,
        >(
            this: *mut ffi::GtkEventControllerKey,
            keyval: libc::c_uint,
            keycode: libc::c_uint,
            state: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), keyval, keycode, from_glib(state))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"key-released\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    key_released_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_modifiers<F: Fn(&EventControllerKey, gdk::ModifierType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn modifiers_trampoline<
            F: Fn(&EventControllerKey, gdk::ModifierType) -> bool + 'static,
        >(
            this: *mut ffi::GtkEventControllerKey,
            object: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"modifiers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    modifiers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EventControllerKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerKey")
    }
}
