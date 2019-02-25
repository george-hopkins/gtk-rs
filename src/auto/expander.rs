// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Expander(Object<ffi::GtkExpander, ffi::GtkExpanderClass, ExpanderClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_expander_get_type(),
    }
}

impl Expander {
    pub fn new(label: Option<&str>) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new(label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new_with_mnemonic(label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_EXPANDER: Option<&Expander> = None;

pub trait ExpanderExt: 'static {
    fn get_expanded(&self) -> bool;

    fn get_label(&self) -> Option<GString>;

    fn get_label_fill(&self) -> bool;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_resize_toplevel(&self) -> bool;

    #[cfg_attr(feature = "v3_20", deprecated)]
    fn get_spacing(&self) -> i32;

    fn get_use_markup(&self) -> bool;

    fn get_use_underline(&self) -> bool;

    fn set_expanded(&self, expanded: bool);

    fn set_label(&self, label: Option<&str>);

    fn set_label_fill(&self, label_fill: bool);

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>);

    fn set_resize_toplevel(&self, resize_toplevel: bool);

    #[cfg_attr(feature = "v3_20", deprecated)]
    fn set_spacing(&self, spacing: i32);

    fn set_use_markup(&self, use_markup: bool);

    fn set_use_underline(&self, use_underline: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_20", deprecated)]
    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Expander>> ExpanderExt for O {
    fn get_expanded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_expanded(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label_fill(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_label_fill(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_resize_toplevel(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_resize_toplevel(self.as_ref().to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_expander_get_spacing(self.as_ref().to_glib_none().0)
        }
    }

    fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_markup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_underline(self.as_ref().to_glib_none().0))
        }
    }

    fn set_expanded(&self, expanded: bool) {
        unsafe {
            ffi::gtk_expander_set_expanded(self.as_ref().to_glib_none().0, expanded.to_glib());
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_expander_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_fill(&self, label_fill: bool) {
        unsafe {
            ffi::gtk_expander_set_label_fill(self.as_ref().to_glib_none().0, label_fill.to_glib());
        }
    }

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>) {
        unsafe {
            ffi::gtk_expander_set_label_widget(self.as_ref().to_glib_none().0, label_widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_resize_toplevel(&self, resize_toplevel: bool) {
        unsafe {
            ffi::gtk_expander_set_resize_toplevel(self.as_ref().to_glib_none().0, resize_toplevel.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_expander_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_use_markup(&self, use_markup: bool) {
        unsafe {
            ffi::gtk_expander_set_use_markup(self.as_ref().to_glib_none().0, use_markup.to_glib());
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_expander_set_use_underline(self.as_ref().to_glib_none().0, use_underline.to_glib());
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_activate(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("activate", &[]).unwrap() };
    }

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::expanded\0".as_ptr() as *const _,
                Some(transmute(notify_expanded_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label-fill\0".as_ptr() as *const _,
                Some(transmute(notify_label_fill_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label-widget\0".as_ptr() as *const _,
                Some(transmute(notify_label_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::resize-toplevel\0".as_ptr() as *const _,
                Some(transmute(notify_resize_toplevel_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute(notify_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute(notify_use_markup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute(notify_use_underline_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_expanded_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_fill_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_resize_toplevel_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_markup_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_underline_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    let f: &F = &*(f as *const F);
    f(&Expander::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Expander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expander")
    }
}
