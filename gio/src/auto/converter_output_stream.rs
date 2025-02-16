// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Converter;
use crate::FilterOutputStream;
use crate::OutputStream;
use crate::PollableOutputStream;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct ConverterOutputStream(Object<ffi::GConverterOutputStream, ffi::GConverterOutputStreamClass>) @extends FilterOutputStream, OutputStream, @implements PollableOutputStream;

    match fn {
        type_ => || ffi::g_converter_output_stream_get_type(),
    }
}

impl ConverterOutputStream {
    #[doc(alias = "g_converter_output_stream_new")]
    pub fn new<P: IsA<OutputStream>, Q: IsA<Converter>>(
        base_stream: &P,
        converter: &Q,
    ) -> ConverterOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_converter_output_stream_new(
                base_stream.as_ref().to_glib_none().0,
                converter.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct ConverterOutputStreamBuilder {
    converter: Option<Converter>,
    base_stream: Option<OutputStream>,
    close_base_stream: Option<bool>,
}

impl ConverterOutputStreamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ConverterOutputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref converter) = self.converter {
            properties.push(("converter", converter));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        let ret = glib::Object::new::<ConverterOutputStream>(&properties).expect("object new");
        ret
    }

    pub fn converter<P: IsA<Converter>>(mut self, converter: &P) -> Self {
        self.converter = Some(converter.clone().upcast());
        self
    }

    pub fn base_stream<P: IsA<OutputStream>>(mut self, base_stream: &P) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub const NONE_CONVERTER_OUTPUT_STREAM: Option<&ConverterOutputStream> = None;

pub trait ConverterOutputStreamExt: 'static {
    #[doc(alias = "g_converter_output_stream_get_converter")]
    fn converter(&self) -> Converter;
}

impl<O: IsA<ConverterOutputStream>> ConverterOutputStreamExt for O {
    fn converter(&self) -> Converter {
        unsafe {
            from_glib_none(ffi::g_converter_output_stream_get_converter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ConverterOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConverterOutputStream")
    }
}
