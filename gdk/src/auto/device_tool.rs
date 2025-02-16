// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AxisFlags;
use crate::DeviceToolType;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct DeviceTool(Object<ffi::GdkDeviceTool>);

    match fn {
        type_ => || ffi::gdk_device_tool_get_type(),
    }
}

impl DeviceTool {
    #[doc(alias = "gdk_device_tool_get_hardware_id")]
    pub fn hardware_id(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_hardware_id(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_tool_get_serial")]
    pub fn serial(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_serial(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_tool_get_tool_type")]
    pub fn tool_type(&self) -> DeviceToolType {
        unsafe { from_glib(ffi::gdk_device_tool_get_tool_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "get_property_axes")]
    pub fn axes(&self) -> AxisFlags {
        unsafe {
            let mut value = glib::Value::from_type(<AxisFlags as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"axes\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `axes` getter")
        }
    }

    pub fn get_property_hardware_id(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"hardware-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `hardware-id` getter")
        }
    }

    pub fn get_property_serial(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"serial\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `serial` getter")
        }
    }

    pub fn get_property_tool_type(&self) -> DeviceToolType {
        unsafe {
            let mut value = glib::Value::from_type(<DeviceToolType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tool-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tool-type` getter")
        }
    }
}

impl fmt::Display for DeviceTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceTool")
    }
}
