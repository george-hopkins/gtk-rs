// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use TreeDragDest;
use TreeDragSource;
use TreeIter;
use TreeModel;
use TreeSortable;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TreeStore(Object<ffi::GtkTreeStore, ffi::GtkTreeStoreClass, TreeStoreClass>) @implements Buildable, TreeDragDest, TreeDragSource, TreeModel, TreeSortable;

    match fn {
        get_type => || ffi::gtk_tree_store_get_type(),
    }
}

impl TreeStore {
    //pub fn new(n_columns: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeStore {
    //    unsafe { TODO: call ffi::gtk_tree_store_new() }
    //}

    //pub fn newv(types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) -> TreeStore {
    //    unsafe { TODO: call ffi::gtk_tree_store_newv() }
    //}
}

pub const NONE_TREE_STORE: Option<&TreeStore> = None;

pub trait TreeStoreExt: 'static {
    fn append(&self, parent: Option<&TreeIter>) -> TreeIter;

    fn clear(&self);

    fn insert(&self, parent: Option<&TreeIter>, position: i32) -> TreeIter;

    fn insert_after(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter;

    fn insert_before(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter;

    //fn insert_with_values(&self, parent: Option<&TreeIter>, position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeIter;

    //fn insert_with_valuesv(&self, parent: Option<&TreeIter>, position: i32, columns: &[i32], values: &[&glib::Value]) -> TreeIter;

    fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool;

    fn iter_depth(&self, iter: &TreeIter) -> i32;

    fn iter_is_valid(&self, iter: &TreeIter) -> bool;

    fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>);

    fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>);

    fn prepend(&self, parent: Option<&TreeIter>) -> TreeIter;

    fn remove(&self, iter: &TreeIter) -> bool;

    //fn reorder(&self, parent: Option<&TreeIter>, new_order: &[i32]);

    //fn set(&self, iter: &TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_column_types(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 });

    //fn set_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn set_valuesv(&self, iter: &TreeIter, columns: &[i32], values: &[&glib::Value]);

    fn swap(&self, a: &TreeIter, b: &TreeIter);
}

impl<O: IsA<TreeStore>> TreeStoreExt for O {
    fn append(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_append(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0));
            iter
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gtk_tree_store_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn insert(&self, parent: Option<&TreeIter>, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), position);
            iter
        }
    }

    fn insert_after(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_after(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    fn insert_before(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_before(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    //fn insert_with_values(&self, parent: Option<&TreeIter>, position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_tree_store_insert_with_values() }
    //}

    //fn insert_with_valuesv(&self, parent: Option<&TreeIter>, position: i32, columns: &[i32], values: &[&glib::Value]) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_tree_store_insert_with_valuesv() }
    //}

    fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_is_ancestor(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(descendant.to_glib_none().0)))
        }
    }

    fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe {
            ffi::gtk_tree_store_iter_depth(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0))
        }
    }

    fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_iter_is_valid(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_after(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(position.to_glib_none().0));
        }
    }

    fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_before(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(position.to_glib_none().0));
        }
    }

    fn prepend(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_prepend(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0));
            iter
        }
    }

    fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_remove(self.as_ref().to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    //fn reorder(&self, parent: Option<&TreeIter>, new_order: &[i32]) {
    //    unsafe { TODO: call ffi::gtk_tree_store_reorder() }
    //}

    //fn set(&self, iter: &TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set() }
    //}

    //fn set_column_types(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_column_types() }
    //}

    //fn set_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_valist() }
    //}

    //fn set_valuesv(&self, iter: &TreeIter, columns: &[i32], values: &[&glib::Value]) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_valuesv() }
    //}

    fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_tree_store_swap(self.as_ref().to_glib_none().0, mut_override(a.to_glib_none().0), mut_override(b.to_glib_none().0));
        }
    }
}

impl fmt::Display for TreeStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeStore")
    }
}
