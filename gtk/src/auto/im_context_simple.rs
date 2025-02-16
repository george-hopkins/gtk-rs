// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::IMContext;
use crate::InputHints;
use crate::InputPurpose;
use glib::object::Cast;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct IMContextSimple(Object<ffi::GtkIMContextSimple, ffi::GtkIMContextSimpleClass>) @extends IMContext;

    match fn {
        type_ => || ffi::gtk_im_context_simple_get_type(),
    }
}

impl IMContextSimple {
    #[doc(alias = "gtk_im_context_simple_new")]
    pub fn new() -> IMContextSimple {
        assert_initialized_main_thread!();
        unsafe { IMContext::from_glib_full(ffi::gtk_im_context_simple_new()).unsafe_cast() }
    }
}

impl Default for IMContextSimple {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct IMContextSimpleBuilder {
    input_hints: Option<InputHints>,
    input_purpose: Option<InputPurpose>,
}

impl IMContextSimpleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> IMContextSimple {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref input_hints) = self.input_hints {
            properties.push(("input-hints", input_hints));
        }
        if let Some(ref input_purpose) = self.input_purpose {
            properties.push(("input-purpose", input_purpose));
        }
        let ret = glib::Object::new::<IMContextSimple>(&properties).expect("object new");
        ret
    }

    pub fn input_hints(mut self, input_hints: InputHints) -> Self {
        self.input_hints = Some(input_hints);
        self
    }

    pub fn input_purpose(mut self, input_purpose: InputPurpose) -> Self {
        self.input_purpose = Some(input_purpose);
        self
    }
}

pub const NONE_IM_CONTEXT_SIMPLE: Option<&IMContextSimple> = None;

impl fmt::Display for IMContextSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IMContextSimple")
    }
}
