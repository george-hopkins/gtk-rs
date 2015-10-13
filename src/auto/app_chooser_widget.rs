// This file was generated by gir (df8a3f3) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Box;
use Buildable;
use Container;
use Orientable;
use Widget;

pub type AppChooserWidget = Object<ffi::GtkAppChooserWidget>;

unsafe impl Upcast<Widget> for AppChooserWidget { }
unsafe impl Upcast<Container> for AppChooserWidget { }
unsafe impl Upcast<Box> for AppChooserWidget { }
unsafe impl Upcast<Buildable> for AppChooserWidget { }
unsafe impl Upcast<Orientable> for AppChooserWidget { }

impl AppChooserWidget {
    pub fn new(content_type: &str) -> AppChooserWidget {
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_widget_new(content_type.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_default_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_widget_get_default_text(self.to_glib_none().0))
        }
    }

    pub fn get_show_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_all(self.to_glib_none().0))
        }
    }

    pub fn get_show_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_default(self.to_glib_none().0))
        }
    }

    pub fn get_show_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_fallback(self.to_glib_none().0))
        }
    }

    pub fn get_show_other(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_other(self.to_glib_none().0))
        }
    }

    pub fn get_show_recommended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_recommended(self.to_glib_none().0))
        }
    }

    pub fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_show_all(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_all(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_default(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_fallback(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_fallback(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_other(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_other(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_recommended(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_recommended(self.to_glib_none().0, setting.to_glib());
        }
    }

}

impl types::StaticType for AppChooserWidget {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_app_chooser_widget_get_type()) }
    }
}
