// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTouchpadPinch(crate::Event);

event_wrapper!(EventTouchpadPinch, GdkEventTouchpadPinch);
event_subtype!(EventTouchpadPinch, ffi::GDK_TOUCHPAD_PINCH);

impl EventTouchpadPinch {
    pub fn is_phase(&self) -> bool {
        unsafe { from_glib(self.as_ref().phase as _) }
    }

    pub fn n_fingers(&self) -> i8 {
        self.as_ref().n_fingers
    }

    pub fn time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    pub fn delta(&self) -> (f64, f64) {
        let dx = self.as_ref().dx;
        let dy = self.as_ref().dy;
        (dx, dy)
    }

    pub fn angle_delta(&self) -> f64 {
        self.as_ref().angle_delta
    }

    pub fn scale(&self) -> f64 {
        self.as_ref().scale
    }

    pub fn root(&self) -> (f64, f64) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }

    pub fn state(&self) -> crate::ModifierType {
        unsafe { from_glib(self.as_ref().state) }
    }
}
