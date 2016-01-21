// This file was generated by gir (baa441b) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
#[cfg(gtk_3_14)]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct GestureRotate(Object<ffi::GtkGestureRotate>): EventController, Gesture;

    match fn {
        get_type => || ffi::gtk_gesture_rotate_get_type(),
    }
}

impl GestureRotate {
    #[cfg(gtk_3_14)]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureRotate {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_rotate_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_14)]
    pub fn get_angle_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_rotate_get_angle_delta(self.to_glib_none().0)
        }
    }
}
