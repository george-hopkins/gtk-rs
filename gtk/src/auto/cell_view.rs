// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::CellArea;
use crate::CellAreaContext;
use crate::CellLayout;
use crate::Container;
use crate::Orientable;
use crate::Orientation;
use crate::TreeModel;
use crate::TreePath;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct CellView(Object<ffi::GtkCellView, ffi::GtkCellViewClass>) @extends Widget, @implements Buildable, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_view_get_type(),
    }
}

impl CellView {
    #[doc(alias = "gtk_cell_view_new")]
    pub fn new() -> CellView {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_cell_view_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_cell_view_new_with_context")]
    pub fn with_context<P: IsA<CellArea>, Q: IsA<CellAreaContext>>(
        area: &P,
        context: &Q,
    ) -> CellView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_context(
                area.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_markup")]
    pub fn with_markup(markup: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_markup(markup.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_pixbuf")]
    pub fn with_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_pixbuf(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_text")]
    pub fn with_text(text: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_text(text.to_glib_none().0))
                .unsafe_cast()
        }
    }
}

impl Default for CellView {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct CellViewBuilder {
    background: Option<String>,
    background_rgba: Option<gdk::RGBA>,
    background_set: Option<bool>,
    cell_area: Option<CellArea>,
    cell_area_context: Option<CellAreaContext>,
    draw_sensitive: Option<bool>,
    fit_model: Option<bool>,
    model: Option<TreeModel>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl CellViewBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CellView {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref background) = self.background {
            properties.push(("background", background));
        }
        if let Some(ref background_rgba) = self.background_rgba {
            properties.push(("background-rgba", background_rgba));
        }
        if let Some(ref background_set) = self.background_set {
            properties.push(("background-set", background_set));
        }
        if let Some(ref cell_area) = self.cell_area {
            properties.push(("cell-area", cell_area));
        }
        if let Some(ref cell_area_context) = self.cell_area_context {
            properties.push(("cell-area-context", cell_area_context));
        }
        if let Some(ref draw_sensitive) = self.draw_sensitive {
            properties.push(("draw-sensitive", draw_sensitive));
        }
        if let Some(ref fit_model) = self.fit_model {
            properties.push(("fit-model", fit_model));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new::<CellView>(&properties).expect("object new");
        ret
    }

    pub fn background(mut self, background: &str) -> Self {
        self.background = Some(background.to_string());
        self
    }

    pub fn background_rgba(mut self, background_rgba: &gdk::RGBA) -> Self {
        self.background_rgba = Some(background_rgba.clone());
        self
    }

    pub fn background_set(mut self, background_set: bool) -> Self {
        self.background_set = Some(background_set);
        self
    }

    pub fn cell_area<P: IsA<CellArea>>(mut self, cell_area: &P) -> Self {
        self.cell_area = Some(cell_area.clone().upcast());
        self
    }

    pub fn cell_area_context<P: IsA<CellAreaContext>>(mut self, cell_area_context: &P) -> Self {
        self.cell_area_context = Some(cell_area_context.clone().upcast());
        self
    }

    pub fn draw_sensitive(mut self, draw_sensitive: bool) -> Self {
        self.draw_sensitive = Some(draw_sensitive);
        self
    }

    pub fn fit_model(mut self, fit_model: bool) -> Self {
        self.fit_model = Some(fit_model);
        self
    }

    pub fn model<P: IsA<TreeModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_CELL_VIEW: Option<&CellView> = None;

pub trait CellViewExt: 'static {
    #[doc(alias = "gtk_cell_view_get_displayed_row")]
    fn displayed_row(&self) -> Option<TreePath>;

    #[doc(alias = "gtk_cell_view_get_draw_sensitive")]
    fn draws_sensitive(&self) -> bool;

    #[doc(alias = "gtk_cell_view_get_fit_model")]
    fn fits_model(&self) -> bool;

    #[doc(alias = "gtk_cell_view_get_model")]
    fn model(&self) -> Option<TreeModel>;

    #[doc(alias = "gtk_cell_view_set_background_rgba")]
    fn set_background_rgba(&self, rgba: &gdk::RGBA);

    #[doc(alias = "gtk_cell_view_set_displayed_row")]
    fn set_displayed_row(&self, path: &mut TreePath);

    #[doc(alias = "gtk_cell_view_set_draw_sensitive")]
    fn set_draw_sensitive(&self, draw_sensitive: bool);

    #[doc(alias = "gtk_cell_view_set_fit_model")]
    fn set_fit_model(&self, fit_model: bool);

    #[doc(alias = "gtk_cell_view_set_model")]
    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>);

    #[doc(alias = "set_property_background")]
    fn set_background(&self, background: Option<&str>);

    #[doc(alias = "get_property_background_rgba")]
    fn background_rgba(&self) -> Option<gdk::RGBA>;

    #[doc(alias = "get_property_background_set")]
    fn is_background_set(&self) -> bool;

    #[doc(alias = "set_property_background_set")]
    fn set_background_set(&self, background_set: bool);

    #[doc(alias = "get_property_cell_area")]
    fn cell_area(&self) -> Option<CellArea>;

    #[doc(alias = "get_property_cell_area_context")]
    fn cell_area_context(&self) -> Option<CellAreaContext>;

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_draw_sensitive_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_fit_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellView>> CellViewExt for O {
    fn displayed_row(&self) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_cell_view_get_displayed_row(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn draws_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_view_get_draw_sensitive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn fits_model(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_view_get_fit_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_cell_view_get_model(self.as_ref().to_glib_none().0)) }
    }

    fn set_background_rgba(&self, rgba: &gdk::RGBA) {
        unsafe {
            ffi::gtk_cell_view_set_background_rgba(
                self.as_ref().to_glib_none().0,
                rgba.to_glib_none().0,
            );
        }
    }

    fn set_displayed_row(&self, path: &mut TreePath) {
        unsafe {
            ffi::gtk_cell_view_set_displayed_row(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
            );
        }
    }

    fn set_draw_sensitive(&self, draw_sensitive: bool) {
        unsafe {
            ffi::gtk_cell_view_set_draw_sensitive(
                self.as_ref().to_glib_none().0,
                draw_sensitive.to_glib(),
            );
        }
    }

    fn set_fit_model(&self, fit_model: bool) {
        unsafe {
            ffi::gtk_cell_view_set_fit_model(self.as_ref().to_glib_none().0, fit_model.to_glib());
        }
    }

    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_cell_view_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_background(&self, background: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"background\0".as_ptr() as *const _,
                background.to_value().to_glib_none().0,
            );
        }
    }

    fn background_rgba(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut value = glib::Value::from_type(<gdk::RGBA as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"background-rgba\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `background-rgba` getter")
        }
    }

    fn is_background_set(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"background-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `background-set` getter")
        }
    }

    fn set_background_set(&self, background_set: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"background-set\0".as_ptr() as *const _,
                background_set.to_value().to_glib_none().0,
            );
        }
    }

    fn cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = glib::Value::from_type(<CellArea as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"cell-area\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `cell-area` getter")
        }
    }

    fn cell_area_context(&self) -> Option<CellAreaContext> {
        unsafe {
            let mut value = glib::Value::from_type(<CellAreaContext as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"cell-area-context\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `cell-area-context` getter")
        }
    }

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellView>,
        {
            let f: &F = &*(f as *const F);
            f(&CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_rgba_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellView>,
        {
            let f: &F = &*(f as *const F);
            f(&CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellView>,
        {
            let f: &F = &*(f as *const F);
            f(&CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_draw_sensitive_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_sensitive_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellView>,
        {
            let f: &F = &*(f as *const F);
            f(&CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-sensitive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_sensitive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_fit_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fit_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellView>,
        {
            let f: &F = &*(f as *const F);
            f(&CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fit-model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fit_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellView>,
        {
            let f: &F = &*(f as *const F);
            f(&CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellView")
    }
}
