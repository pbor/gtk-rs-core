// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use TlsDatabase;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TlsFileDatabase(Interface<ffi::GTlsFileDatabase>) @requires TlsDatabase;

    match fn {
        get_type => || ffi::g_tls_file_database_get_type(),
    }
}

impl TlsFileDatabase {
    pub fn new<P: AsRef<std::path::Path>>(anchors: P) -> Result<TlsFileDatabase, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_file_database_new(anchors.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub const NONE_TLS_FILE_DATABASE: Option<&TlsFileDatabase> = None;

pub trait TlsFileDatabaseExt: 'static {
    fn get_property_anchors(&self) -> Option<GString>;

    fn set_property_anchors<'a, P: Into<Option<&'a str>>>(&self, anchors: P);

    fn connect_property_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsFileDatabase>> TlsFileDatabaseExt for O {
    fn get_property_anchors(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"anchors\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_anchors<'a, P: Into<Option<&'a str>>>(&self, anchors: P) {
        let anchors = anchors.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"anchors\0".as_ptr() as *const _, Value::from(anchors).to_glib_none().0);
        }
    }

    fn connect_property_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::anchors\0".as_ptr() as *const _,
                Some(transmute(notify_anchors_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_anchors_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GTlsFileDatabase, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsFileDatabase> {
    let f: &F = transmute(f);
    f(&TlsFileDatabase::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for TlsFileDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsFileDatabase")
    }
}
