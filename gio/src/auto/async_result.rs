// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct AsyncResult(Interface<ffi::GAsyncResult, ffi::GAsyncResultIface>);

    match fn {
        type_ => || ffi::g_async_result_get_type(),
    }
}

pub const NONE_ASYNC_RESULT: Option<&AsyncResult> = None;

pub trait AsyncResultExt: 'static {
    #[doc(alias = "g_async_result_get_source_object")]
    fn source_object(&self) -> Option<glib::Object>;

    //#[doc(alias = "g_async_result_get_user_data")]
    //fn user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    //#[doc(alias = "g_async_result_is_tagged")]
    //fn is_tagged(&self, source_tag: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

    #[doc(alias = "g_async_result_legacy_propagate_error")]
    fn legacy_propagate_error(&self) -> Result<(), glib::Error>;
}

impl<O: IsA<AsyncResult>> AsyncResultExt for O {
    fn source_object(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::g_async_result_get_source_object(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:g_async_result_get_user_data() }
    //}

    //fn is_tagged(&self, source_tag: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:g_async_result_is_tagged() }
    //}

    fn legacy_propagate_error(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_async_result_legacy_propagate_error(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for AsyncResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AsyncResult")
    }
}
