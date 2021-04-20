// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::BaselinePosition;
use crate::Box;
use crate::Buildable;
use crate::Container;
use crate::Orientable;
use crate::Orientation;
use crate::ResizeMode;
use crate::ShortcutType;
use crate::SizeGroup;
use crate::TextDirection;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ShortcutsShortcut(Object<ffi::GtkShortcutsShortcut, ffi::GtkShortcutsShortcutClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_shortcuts_shortcut_get_type(),
    }
}

impl ShortcutsShortcut {
    #[doc(alias = "set_property_accel_size_group")]
    pub fn set_accel_size_group<P: IsA<SizeGroup>>(&self, accel_size_group: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accel-size-group\0".as_ptr() as *const _,
                glib::Value::from(accel_size_group).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_accelerator")]
    pub fn accelerator(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accelerator\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `accelerator` getter")
        }
    }

    #[doc(alias = "set_property_accelerator")]
    pub fn set_accelerator(&self, accelerator: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accelerator\0".as_ptr() as *const _,
                glib::Value::from(accelerator).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "get_property_action_name")]
    pub fn action_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"action-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `action-name` getter")
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "set_property_action_name")]
    pub fn set_action_name(&self, action_name: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"action-name\0".as_ptr() as *const _,
                glib::Value::from(action_name).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_icon")]
    pub fn icon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = glib::Value::from_type(<gio::Icon as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon` getter")
        }
    }

    #[doc(alias = "set_property_icon")]
    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon\0".as_ptr() as *const _,
                glib::Value::from(icon).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_icon_set")]
    pub fn is_icon_set(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-set` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_icon_set")]
    pub fn set_icon_set(&self, icon_set: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon-set\0".as_ptr() as *const _,
                glib::Value::from(&icon_set).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_shortcut_type")]
    pub fn shortcut_type(&self) -> ShortcutType {
        unsafe {
            let mut value = glib::Value::from_type(<ShortcutType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"shortcut-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shortcut-type` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_shortcut_type")]
    pub fn set_shortcut_type(&self, shortcut_type: ShortcutType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"shortcut-type\0".as_ptr() as *const _,
                glib::Value::from(&shortcut_type).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_subtitle")]
    pub fn subtitle(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"subtitle\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `subtitle` getter")
        }
    }

    #[doc(alias = "set_property_subtitle")]
    pub fn set_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"subtitle\0".as_ptr() as *const _,
                glib::Value::from(subtitle).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_subtitle_set")]
    pub fn is_subtitle_set(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"subtitle-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `subtitle-set` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_subtitle_set")]
    pub fn set_subtitle_set(&self, subtitle_set: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"subtitle-set\0".as_ptr() as *const _,
                glib::Value::from(&subtitle_set).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `title` getter")
        }
    }

    #[doc(alias = "set_property_title")]
    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                glib::Value::from(title).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "set_property_title_size_group")]
    pub fn set_title_size_group<P: IsA<SizeGroup>>(&self, title_size_group: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"title-size-group\0".as_ptr() as *const _,
                glib::Value::from(title_size_group).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_accel_size_group_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_size_group_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accel-size-group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accel_size_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_accelerator_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accelerator_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accelerator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accelerator_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn connect_property_action_name_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_name_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_direction_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_icon_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_icon_set_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_set_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_shortcut_type_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcut_type_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shortcut-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcut_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_subtitle_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_subtitle_set_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_set_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_size_group_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_size_group_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut ffi::GtkShortcutsShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title-size-group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_size_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct ShortcutsShortcutBuilder {
    accel_size_group: Option<SizeGroup>,
    accelerator: Option<String>,
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    action_name: Option<String>,
    direction: Option<TextDirection>,
    icon: Option<gio::Icon>,
    icon_set: Option<bool>,
    shortcut_type: Option<ShortcutType>,
    subtitle: Option<String>,
    subtitle_set: Option<bool>,
    title: Option<String>,
    title_size_group: Option<SizeGroup>,
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
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

impl ShortcutsShortcutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ShortcutsShortcut {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accel_size_group) = self.accel_size_group {
            properties.push(("accel-size-group", accel_size_group));
        }
        if let Some(ref accelerator) = self.accelerator {
            properties.push(("accelerator", accelerator));
        }
        #[cfg(any(feature = "v3_22", feature = "dox"))]
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        if let Some(ref direction) = self.direction {
            properties.push(("direction", direction));
        }
        if let Some(ref icon) = self.icon {
            properties.push(("icon", icon));
        }
        if let Some(ref icon_set) = self.icon_set {
            properties.push(("icon-set", icon_set));
        }
        if let Some(ref shortcut_type) = self.shortcut_type {
            properties.push(("shortcut-type", shortcut_type));
        }
        if let Some(ref subtitle) = self.subtitle {
            properties.push(("subtitle", subtitle));
        }
        if let Some(ref subtitle_set) = self.subtitle_set {
            properties.push(("subtitle-set", subtitle_set));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref title_size_group) = self.title_size_group {
            properties.push(("title-size-group", title_size_group));
        }
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
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
        let ret = glib::Object::new::<ShortcutsShortcut>(&properties).expect("object new");
        ret
    }

    pub fn accel_size_group<P: IsA<SizeGroup>>(mut self, accel_size_group: &P) -> Self {
        self.accel_size_group = Some(accel_size_group.clone().upcast());
        self
    }

    pub fn accelerator(mut self, accelerator: &str) -> Self {
        self.accelerator = Some(accelerator.to_string());
        self
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }

    pub fn direction(mut self, direction: TextDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn icon<P: IsA<gio::Icon>>(mut self, icon: &P) -> Self {
        self.icon = Some(icon.clone().upcast());
        self
    }

    pub fn icon_set(mut self, icon_set: bool) -> Self {
        self.icon_set = Some(icon_set);
        self
    }

    pub fn shortcut_type(mut self, shortcut_type: ShortcutType) -> Self {
        self.shortcut_type = Some(shortcut_type);
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn subtitle_set(mut self, subtitle_set: bool) -> Self {
        self.subtitle_set = Some(subtitle_set);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn title_size_group<P: IsA<SizeGroup>>(mut self, title_size_group: &P) -> Self {
        self.title_size_group = Some(title_size_group.clone().upcast());
        self
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
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

impl fmt::Display for ShortcutsShortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ShortcutsShortcut")
    }
}
