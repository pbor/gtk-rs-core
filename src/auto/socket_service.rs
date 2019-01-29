// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SocketConnection;
use SocketListener;
use ffi;
use glib;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct SocketService(Object<ffi::GSocketService, ffi::GSocketServiceClass, SocketServiceClass>) @extends SocketListener;

    match fn {
        get_type => || ffi::g_socket_service_get_type(),
    }
}

impl SocketService {
    pub fn new() -> SocketService {
        unsafe {
            from_glib_full(ffi::g_socket_service_new())
        }
    }
}

impl Default for SocketService {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SOCKET_SERVICE: Option<&SocketService> = None;

pub trait SocketServiceExt: 'static {
    fn is_active(&self) -> bool;

    fn start(&self);

    fn stop(&self);

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_property_active(&self) -> bool;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn set_property_active(&self, active: bool);

    fn connect_incoming<F: Fn(&Self, &SocketConnection, &Option<glib::Object>) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketService>> SocketServiceExt for O {
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_service_is_active(self.as_ref().to_glib_none().0))
        }
    }

    fn start(&self) {
        unsafe {
            ffi::g_socket_service_start(self.as_ref().to_glib_none().0);
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::g_socket_service_stop(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"active\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"active\0".as_ptr() as *const _, Value::from(&active).to_glib_none().0);
        }
    }

    fn connect_incoming<F: Fn(&Self, &SocketConnection, &Option<glib::Object>) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"incoming\0".as_ptr() as *const _,
                Some(transmute(incoming_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn incoming_trampoline<P, F: Fn(&P, &SocketConnection, &Option<glib::Object>) -> bool + 'static>(this: *mut ffi::GSocketService, connection: *mut ffi::GSocketConnection, source_object: *mut gobject_ffi::GObject, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<SocketService> {
    let f: &F = transmute(f);
    f(&SocketService::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(connection), &from_glib_borrow(source_object)).to_glib()
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketService, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketService> {
    let f: &F = transmute(f);
    f(&SocketService::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SocketService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketService")
    }
}
