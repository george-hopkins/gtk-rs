// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventCrossing(crate::Event);

event_wrapper!(EventCrossing, GdkEventCrossing);
event_subtype!(EventCrossing, ffi::GDK_ENTER_NOTIFY | ffi::GDK_LEAVE_NOTIFY);

impl EventCrossing {
    pub fn position(&self) -> (f64, f64) {
        let x = self.as_ref().x;
        let y = self.as_ref().y;
        (x, y)
    }

    pub fn subwindow(&self) -> Option<crate::Window> {
        unsafe { from_glib_none(self.as_ref().subwindow) }
    }

    pub fn mode(&self) -> crate::CrossingMode {
        unsafe { from_glib(self.as_ref().mode) }
    }

    pub fn detail(&self) -> crate::NotifyType {
        unsafe { from_glib(self.as_ref().detail) }
    }

    pub fn state(&self) -> crate::ModifierType {
        unsafe { from_glib(self.as_ref().state) }
    }

    pub fn time(&self) -> u32 {
        self.as_ref().time
    }

    pub fn root(&self) -> (f64, f64) {
        let x_root = self.as_ref().x_root;
        let y_root = self.as_ref().y_root;
        (x_root, y_root)
    }

    pub fn gets_focus(&self) -> bool {
        unsafe { from_glib(self.as_ref().focus) }
    }
}
