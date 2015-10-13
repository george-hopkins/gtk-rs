// This file was generated by gir (df8a3f3) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Actionable;
use Bin;
use Buildable;
use Container;
use ToolItem;
use Widget;

pub type ToolButton = Object<ffi::GtkToolButton>;

unsafe impl Upcast<Widget> for ToolButton { }
unsafe impl Upcast<Container> for ToolButton { }
unsafe impl Upcast<Bin> for ToolButton { }
unsafe impl Upcast<ToolItem> for ToolButton { }
unsafe impl Upcast<Actionable> for ToolButton { }
unsafe impl Upcast<Buildable> for ToolButton { }

impl ToolButton {
    pub fn new<T: Upcast<Widget>>(icon_widget: Option<&T>, label: Option<&str>) -> ToolButton {
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new(icon_widget.map(Upcast::upcast).to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> ToolButton {
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }

}

impl types::StaticType for ToolButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_tool_button_get_type()) }
    }
}

pub trait ToolButtonExt {
    fn get_icon_name(&self) -> Option<String>;
    fn get_icon_widget(&self) -> Option<Widget>;
    fn get_label(&self) -> Option<String>;
    fn get_label_widget(&self) -> Option<Widget>;
    fn get_stock_id(&self) -> Option<String>;
    fn get_use_underline(&self) -> bool;
    fn set_icon_name(&self, icon_name: Option<&str>);
    fn set_icon_widget<T: Upcast<Widget>>(&self, icon_widget: Option<&T>);
    fn set_label(&self, label: Option<&str>);
    fn set_label_widget<T: Upcast<Widget>>(&self, label_widget: Option<&T>);
    fn set_stock_id(&self, stock_id: Option<&str>);
    fn set_use_underline(&self, use_underline: bool);
}

impl<O: Upcast<ToolButton>> ToolButtonExt for O {
    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_name(self.upcast().to_glib_none().0))
        }
    }

    fn get_icon_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_widget(self.upcast().to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label(self.upcast().to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label_widget(self.upcast().to_glib_none().0))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_stock_id(self.upcast().to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_button_get_use_underline(self.upcast().to_glib_none().0))
        }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_icon_name(self.upcast().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_icon_widget<T: Upcast<Widget>>(&self, icon_widget: Option<&T>) {
        unsafe {
            ffi::gtk_tool_button_set_icon_widget(self.upcast().to_glib_none().0, icon_widget.map(Upcast::upcast).to_glib_none().0);
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_label(self.upcast().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_widget<T: Upcast<Widget>>(&self, label_widget: Option<&T>) {
        unsafe {
            ffi::gtk_tool_button_set_label_widget(self.upcast().to_glib_none().0, label_widget.map(Upcast::upcast).to_glib_none().0);
        }
    }

    fn set_stock_id(&self, stock_id: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_stock_id(self.upcast().to_glib_none().0, stock_id.to_glib_none().0);
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_tool_button_set_use_underline(self.upcast().to_glib_none().0, use_underline.to_glib());
        }
    }

}
