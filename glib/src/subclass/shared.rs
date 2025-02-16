// Take a look at the license at the top of the repository in the LICENSE file.

//! Module for registering shared types for Rust types.

use crate::translate::*;

pub unsafe trait RefCounted: Clone + Sized + 'static {
    /// The inner type
    type InnerType;

    /// The function used to increment the inner type refcount
    unsafe fn ref_(this: *const Self::InnerType) -> *const Self::InnerType;

    /// Converts the RefCounted object to a raw pointer to InnerType
    unsafe fn into_raw(self) -> *const Self::InnerType;

    /// Converts a raw pointer to InnerType to a RefCounted object
    unsafe fn from_raw(this: *const Self::InnerType) -> Self;
}

unsafe impl<T> RefCounted for std::sync::Arc<T>
where
    T: 'static,
{
    type InnerType = T;

    unsafe fn ref_(this: *const Self::InnerType) -> *const Self::InnerType {
        std::sync::Arc::increment_strong_count(this);
        this
    }

    unsafe fn into_raw(self) -> *const Self::InnerType {
        std::sync::Arc::into_raw(self)
    }

    unsafe fn from_raw(this: *const Self::InnerType) -> Self {
        std::sync::Arc::from_raw(this)
    }
}

unsafe impl<T> RefCounted for std::rc::Rc<T>
where
    T: 'static,
{
    type InnerType = T;

    unsafe fn ref_(this: *const Self::InnerType) -> *const Self::InnerType {
        use std::mem::ManuallyDrop;
        let this_rc = ManuallyDrop::new(std::rc::Rc::from_raw(this));
        std::rc::Rc::into_raw(ManuallyDrop::take(&mut this_rc.clone()))
    }

    unsafe fn into_raw(self) -> *const Self::InnerType {
        std::rc::Rc::into_raw(self)
    }

    unsafe fn from_raw(this: *const Self::InnerType) -> Self {
        std::rc::Rc::from_raw(this)
    }
}

/// Trait for defining shared types.
///
/// Links together the type name with the type itself.
///
/// See [`register_shared_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_shared_type`]: fn.register_shared_type.html
pub trait SharedType: Clone + Sized + 'static {
    /// Shared type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// The inner refcounted type
    type RefCountedType: RefCounted;

    /// Returns the type ID.
    ///
    /// This is usually defined via the [`Shared!`] derive macro.
    ///
    /// [`Shared!`]: ../../derive.Shared.html
    fn type_() -> crate::Type;

    /// Converts the SharedType into its inner RefCountedType
    fn into_refcounted(self) -> Self::RefCountedType;

    /// Constructs a SharedType from a RefCountedType
    fn from_refcounted(this: Self::RefCountedType) -> Self;
}

/// Register a boxed `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// See [`Shared!`] for defining a function that ensures that
/// this is only called once and returns the type id.
///
/// [`Shared!`]: ../../derive.Shared.html
pub fn register_shared_type<T: SharedType>() -> crate::Type {
    unsafe {
        use std::ffi::CString;
        unsafe extern "C" fn shared_ref<T: SharedType>(v: ffi::gpointer) -> ffi::gpointer {
            T::RefCountedType::ref_(v as *const <T::RefCountedType as RefCounted>::InnerType)
                as ffi::gpointer
        }
        unsafe extern "C" fn shared_unref<T: SharedType>(v: ffi::gpointer) {
            let _ = T::RefCountedType::from_raw(
                v as *const <T::RefCountedType as RefCounted>::InnerType,
            );
        }

        let type_name = CString::new(T::NAME).unwrap();
        if gobject_ffi::g_type_from_name(type_name.as_ptr()) != gobject_ffi::G_TYPE_INVALID {
            panic!(
                "Type {} has already been registered",
                type_name.to_str().unwrap()
            );
        }

        from_glib(gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(shared_ref::<T>),
            Some(shared_unref::<T>),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // We rename the current crate as glib, since the macros in glib-macros
    // generate the glib namespace through the crate_ident_new utility,
    // and that returns `glib` (and not `crate`) when called inside the glib crate
    use crate as glib;
    use crate::value::ToValue;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MySharedInner {
        foo: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq, glib::GSharedBoxed)]
    #[gshared_boxed(type_name = "MySharedArc")]
    struct MySharedArc(std::sync::Arc<MySharedInner>);

    #[derive(Clone, Debug, PartialEq, Eq, glib::GSharedBoxed)]
    #[gshared_boxed(type_name = "MySharedRc")]
    struct MySharedRc(std::rc::Rc<MySharedInner>);

    #[test]
    fn test_register() {
        assert_ne!(crate::Type::INVALID, MySharedArc::type_());
        assert_ne!(crate::Type::INVALID, MySharedRc::type_());
    }

    #[test]
    fn test_value_arc() {
        assert_ne!(crate::Type::INVALID, MySharedArc::type_());

        let b = MySharedArc::from_refcounted(std::sync::Arc::new(MySharedInner {
            foo: String::from("abc"),
        }));
        let v = b.to_value();
        let b2 = v.get::<MySharedArc>().unwrap();
        assert!(std::sync::Arc::ptr_eq(&b.0, &b2.0));
    }

    #[test]
    fn test_value_rc() {
        assert_ne!(crate::Type::INVALID, MySharedRc::type_());

        let b = MySharedRc::from_refcounted(std::rc::Rc::new(MySharedInner {
            foo: String::from("abc"),
        }));
        let v = b.to_value();
        let b2 = v.get::<MySharedRc>().unwrap();
        assert!(std::rc::Rc::ptr_eq(&b.0, &b2.0));
    }

    #[test]
    fn same_ffi_pointer_arc() {
        assert_ne!(crate::Type::INVALID, MySharedArc::type_());

        let b = MySharedArc::from_refcounted(std::sync::Arc::new(MySharedInner {
            foo: String::from("abc"),
        }));

        let inner_raw_ptr = std::sync::Arc::into_raw(b.clone().0);

        assert_eq!(std::sync::Arc::strong_count(&b.0), 2);

        let inner_raw_ptr_clone =
            unsafe { <MySharedArc as SharedType>::RefCountedType::ref_(inner_raw_ptr) };

        assert_eq!(std::sync::Arc::strong_count(&b.0), 3);
        assert!(std::ptr::eq(inner_raw_ptr, inner_raw_ptr_clone));

        let _ = unsafe { <MySharedArc as SharedType>::RefCountedType::from_raw(inner_raw_ptr) };
        let _ =
            unsafe { <MySharedArc as SharedType>::RefCountedType::from_raw(inner_raw_ptr_clone) };
        assert_eq!(std::sync::Arc::strong_count(&b.0), 1);
    }

    #[test]
    fn same_ffi_pointer_rc() {
        assert_ne!(crate::Type::INVALID, MySharedRc::type_());

        let b = MySharedRc::from_refcounted(std::rc::Rc::new(MySharedInner {
            foo: String::from("abc"),
        }));

        let inner_raw_ptr = std::rc::Rc::into_raw(b.clone().0);

        assert_eq!(std::rc::Rc::strong_count(&b.0), 2);

        let inner_raw_ptr_clone =
            unsafe { <MySharedRc as SharedType>::RefCountedType::ref_(inner_raw_ptr) };

        assert_eq!(std::rc::Rc::strong_count(&b.0), 3);
        assert!(std::ptr::eq(inner_raw_ptr, inner_raw_ptr_clone));

        let _ = unsafe { <MySharedRc as SharedType>::RefCountedType::from_raw(inner_raw_ptr) };
        let _ =
            unsafe { <MySharedRc as SharedType>::RefCountedType::from_raw(inner_raw_ptr_clone) };
        assert_eq!(std::rc::Rc::strong_count(&b.0), 1);
    }
}
