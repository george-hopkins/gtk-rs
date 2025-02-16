// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Box;
use crate::Plane;
use crate::Point3D;
use crate::Vec2;
use crate::Vec3;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Triangle(Boxed<ffi::graphene_triangle_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_triangle_get_type(), ptr as *mut _) as *mut ffi::graphene_triangle_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_triangle_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        type_ => || ffi::graphene_triangle_get_type(),
    }
}

impl Triangle {
    #[doc(alias = "graphene_triangle_contains_point")]
    pub fn contains_point(&self, p: &Point3D) -> bool {
        unsafe {
            from_glib(ffi::graphene_triangle_contains_point(
                self.to_glib_none().0,
                p.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_triangle_equal")]
    fn equal(&self, b: &Triangle) -> bool {
        unsafe {
            from_glib(ffi::graphene_triangle_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_triangle_get_area")]
    pub fn area(&self) -> f32 {
        unsafe { ffi::graphene_triangle_get_area(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_triangle_get_barycoords")]
    pub fn barycoords(&self, p: Option<&Point3D>) -> Option<Vec2> {
        unsafe {
            let mut res = Vec2::uninitialized();
            let ret = from_glib(ffi::graphene_triangle_get_barycoords(
                self.to_glib_none().0,
                p.to_glib_none().0,
                res.to_glib_none_mut().0,
            ));
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }

    #[doc(alias = "graphene_triangle_get_bounding_box")]
    pub fn bounding_box(&self) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_triangle_get_bounding_box(
                self.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_triangle_get_midpoint")]
    pub fn midpoint(&self) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_triangle_get_midpoint(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_triangle_get_normal")]
    pub fn normal(&self) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            ffi::graphene_triangle_get_normal(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_triangle_get_plane")]
    pub fn plane(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_triangle_get_plane(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_triangle_get_points")]
    pub fn points(&self) -> (Point3D, Point3D, Point3D) {
        unsafe {
            let mut a = Point3D::uninitialized();
            let mut b = Point3D::uninitialized();
            let mut c = Point3D::uninitialized();
            ffi::graphene_triangle_get_points(
                self.to_glib_none().0,
                a.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
                c.to_glib_none_mut().0,
            );
            (a, b, c)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "graphene_triangle_get_uv")]
    pub fn uv(&self, p: Option<&Point3D>, uv_a: &Vec2, uv_b: &Vec2, uv_c: &Vec2) -> Option<Vec2> {
        unsafe {
            let mut res = Vec2::uninitialized();
            let ret = from_glib(ffi::graphene_triangle_get_uv(
                self.to_glib_none().0,
                p.to_glib_none().0,
                uv_a.to_glib_none().0,
                uv_b.to_glib_none().0,
                uv_c.to_glib_none().0,
                res.to_glib_none_mut().0,
            ));
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }

    #[doc(alias = "graphene_triangle_get_vertices")]
    pub fn vertices(&self) -> (Vec3, Vec3, Vec3) {
        unsafe {
            let mut a = Vec3::uninitialized();
            let mut b = Vec3::uninitialized();
            let mut c = Vec3::uninitialized();
            ffi::graphene_triangle_get_vertices(
                self.to_glib_none().0,
                a.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
                c.to_glib_none_mut().0,
            );
            (a, b, c)
        }
    }

    //#[cfg(any(feature = "v1_10", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    //#[doc(alias = "graphene_triangle_init_from_float")]
    //pub fn init_from_float(&mut self, a: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 3, b: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 3, c: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 3) -> Option<Triangle> {
    //    unsafe { TODO: call ffi:graphene_triangle_init_from_float() }
    //}

    #[doc(alias = "graphene_triangle_init_from_point3d")]
    pub fn init_from_point3d(
        &mut self,
        a: Option<&Point3D>,
        b: Option<&Point3D>,
        c: Option<&Point3D>,
    ) {
        unsafe {
            ffi::graphene_triangle_init_from_point3d(
                self.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_triangle_init_from_vec3")]
    pub fn init_from_vec3(&mut self, a: Option<&Vec3>, b: Option<&Vec3>, c: Option<&Vec3>) {
        unsafe {
            ffi::graphene_triangle_init_from_vec3(
                self.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
        }
    }
}

impl PartialEq for Triangle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Triangle {}
