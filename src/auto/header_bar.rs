// This file was generated by gir (df8a3f3) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Buildable;
use Container;
use Widget;

pub type HeaderBar = Object<ffi::GtkHeaderBar>;

unsafe impl Upcast<Widget> for HeaderBar { }
unsafe impl Upcast<Container> for HeaderBar { }
unsafe impl Upcast<Buildable> for HeaderBar { }

impl HeaderBar {
    #[cfg(gtk_3_10)]
    pub fn new() -> HeaderBar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_header_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_custom_title(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_custom_title(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_decoration_layout(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_decoration_layout(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_has_subtitle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_has_subtitle(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_subtitle(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_subtitle(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_title(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn pack_end<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_end(self.to_glib_none().0, child.upcast().to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn pack_start<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_start(self.to_glib_none().0, child.upcast().to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_custom_title<T: Upcast<Widget>>(&self, title_widget: Option<&T>) {
        unsafe {
            ffi::gtk_header_bar_set_custom_title(self.to_glib_none().0, title_widget.map(Upcast::upcast).to_glib_none().0);
        }
    }

    #[cfg(gtk_3_12)]
    pub fn set_decoration_layout(&self, layout: Option<&str>) {
        unsafe {
            ffi::gtk_header_bar_set_decoration_layout(self.to_glib_none().0, layout.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_12)]
    pub fn set_has_subtitle(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_has_subtitle(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_show_close_button(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            ffi::gtk_header_bar_set_subtitle(self.to_glib_none().0, subtitle.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::gtk_header_bar_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

}

impl types::StaticType for HeaderBar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_header_bar_get_type()) }
    }
}
