// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;

use crate::utils::{crate_ident_new, find_attribute_meta, find_nested_meta, parse_type_name};

fn gen_option_to_ptr() -> TokenStream {
    quote! {
        match s {
            Some(s) => Box::into_raw(Box::new(s.clone())),
            None => std::ptr::null_mut(),
        };
    }
}

fn gen_impl_from_value_optional(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    quote! {
        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a #crate_ident::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                *Box::from_raw(ptr as *mut #name)
            }
        }

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for &'a #name {
            type Checker = #crate_ident::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a #crate_ident::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_get_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                &*(ptr as *mut #name)
            }
        }
    }
}

fn gen_impl_from_value(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    quote! {
        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a #crate_ident::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                *Box::from_raw(ptr as *mut #name)
            }
        }

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for &'a #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a #crate_ident::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_get_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                &*(ptr as *mut #name)
            }
        }
    }
}

fn gen_impl_to_value_optional(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    let option_to_ptr = gen_option_to_ptr();

    quote! {
        impl #crate_ident::value::ToValueOptional for #name {
            fn to_value_optional(s: Option<&Self>) -> #crate_ident::Value {
                let mut value = #crate_ident::Value::for_value_type::<#name>();
                unsafe {
                    let ptr: *mut #name = #option_to_ptr;
                    #crate_ident::gobject_ffi::g_value_take_boxed(
                        #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        ptr as *mut _
                    );
                }

                value
            }
        }
    }
}

pub fn impl_gboxed(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let gtype_name = match parse_type_name(&input, "gboxed") {
        Ok(v) => v,
        Err(e) => abort_call_site!(
            "{}: derive(GBoxed) requires #[gboxed(type_name = \"BoxedTypeName\")]",
            e
        ),
    };

    let crate_ident = crate_ident_new();

    let meta = find_attribute_meta(&input.attrs, "gboxed")
        .unwrap()
        .unwrap();
    let nullable = find_nested_meta(&meta, "nullable").is_some();

    let impl_from_value = if !nullable {
        gen_impl_from_value(name, &crate_ident)
    } else {
        gen_impl_from_value_optional(name, &crate_ident)
    };
    let impl_to_value_optional = if nullable {
        gen_impl_to_value_optional(name, &crate_ident)
    } else {
        quote! {}
    };

    quote! {
        impl #crate_ident::subclass::boxed::BoxedType for #name {
            const NAME: &'static str = #gtype_name;

            fn type_() -> #crate_ident::Type {
                static mut TYPE_: #crate_ident::Type = #crate_ident::Type::INVALID;
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();

                ONCE.call_once(|| {
                    let type_ = #crate_ident::subclass::register_boxed_type::<Self>();
                    unsafe {
                        TYPE_ = type_;
                    }
                });

                unsafe {
                    assert!(TYPE_.is_valid());
                    TYPE_
                }
            }
        }

        impl #crate_ident::StaticType for #name {
            fn static_type() -> #crate_ident::Type {
                <#name as #crate_ident::subclass::boxed::BoxedType>::type_()
            }
        }

        impl #crate_ident::value::ValueType for #name {
            type Type = #name;
        }

        impl #crate_ident::value::ToValue for #name {
            fn to_value(&self) -> #crate_ident::Value {
                unsafe {
                    let ptr: *mut #name = Box::into_raw(Box::new(self.clone()));
                    let mut value = #crate_ident::Value::from_type(<#name as #crate_ident::StaticType>::static_type());
                    #crate_ident::gobject_ffi::g_value_take_boxed(
                        #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        ptr as *mut _
                    );
                    value
                }
            }

            fn value_type(&self) -> #crate_ident::Type {
                <#name as #crate_ident::StaticType>::static_type()
            }
        }

        #impl_to_value_optional

        #impl_from_value
    }
}
