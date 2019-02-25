// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use CheckButton;
use Container;
use ToggleButton;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct RadioButton(Object<ffi::GtkRadioButton, ffi::GtkRadioButtonClass, RadioButtonClass>) @extends CheckButton, ToggleButton, Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_button_get_type(),
    }
}

impl RadioButton {
    pub fn new_from_widget<P: IsA<RadioButton>>(radio_group_member: &P) -> RadioButton {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_from_widget(radio_group_member.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_label_from_widget<P: IsA<RadioButton>>(radio_group_member: &P, label: &str) -> RadioButton {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label_from_widget(radio_group_member.as_ref().to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic_from_widget<P: IsA<RadioButton>>(radio_group_member: &P, label: &str) -> RadioButton {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic_from_widget(radio_group_member.as_ref().to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_RADIO_BUTTON: Option<&RadioButton> = None;

pub trait RadioButtonExt: 'static {
    fn get_group(&self) -> Vec<RadioButton>;

    fn join_group<P: IsA<RadioButton>>(&self, group_source: Option<&P>);

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RadioButton>> RadioButtonExt for O {
    fn get_group(&self) -> Vec<RadioButton> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_button_get_group(self.as_ref().to_glib_none().0))
        }
    }

    fn join_group<P: IsA<RadioButton>>(&self, group_source: Option<&P>) {
        unsafe {
            ffi::gtk_radio_button_join_group(self.as_ref().to_glib_none().0, group_source.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"group-changed\0".as_ptr() as *const _,
                Some(transmute(group_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn group_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkRadioButton, f: glib_ffi::gpointer)
where P: IsA<RadioButton> {
    let f: &F = &*(f as *const F);
    f(&RadioButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for RadioButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RadioButton")
    }
}
