// This file was generated by gir (b798f4f) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use gdk;
use glib::translate::*;

glib_wrapper! {
    pub struct AccelGroup(Object<ffi::GtkAccelGroup>);

    match fn {
        get_type => || ffi::gtk_accel_group_get_type(),
    }
}

impl AccelGroup {
    pub fn new() -> AccelGroup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_accel_group_new())
        }
    }

    //pub fn activate<T: IsA</*Ignored*/glib::Object>>(&self, accel_quark: /*Ignored*/glib::Quark, acceleratable: &T, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
    //    unsafe { TODO: call ffi::gtk_accel_group_activate() }
    //}

    //pub fn connect(&self, accel_key: u32, accel_mods: gdk::ModifierType, accel_flags: AccelFlags, closure: /*Ignored*/&glib::Closure) {
    //    unsafe { TODO: call ffi::gtk_accel_group_connect() }
    //}

    //pub fn connect_by_path(&self, accel_path: &str, closure: /*Ignored*/&glib::Closure) {
    //    unsafe { TODO: call ffi::gtk_accel_group_connect_by_path() }
    //}

    //pub fn disconnect(&self, closure: /*Ignored*/Option<&glib::Closure>) -> bool {
    //    unsafe { TODO: call ffi::gtk_accel_group_disconnect() }
    //}

    pub fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_disconnect_key(self.to_glib_none().0, accel_key, accel_mods.to_glib()))
        }
    }

    //pub fn find(&self, find_func: /*Unknown conversion*//*Unimplemented*/AccelGroupFindFunc, data: /*Unimplemented*/Fundamental: Pointer) -> /*Ignored*/Option<AccelKey> {
    //    unsafe { TODO: call ffi::gtk_accel_group_find() }
    //}

    pub fn get_is_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_get_is_locked(self.to_glib_none().0))
        }
    }

    pub fn get_modifier_mask(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_accel_group_get_modifier_mask(self.to_glib_none().0))
        }
    }

    pub fn lock(&self) {
        unsafe {
            ffi::gtk_accel_group_lock(self.to_glib_none().0);
        }
    }

    pub fn unlock(&self) {
        unsafe {
            ffi::gtk_accel_group_unlock(self.to_glib_none().0);
        }
    }

    //pub fn from_accel_closure(closure: /*Ignored*/&glib::Closure) -> Option<AccelGroup> {
    //    unsafe { TODO: call ffi::gtk_accel_group_from_accel_closure() }
    //}

    //pub fn connect_accel_activate<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    TODO: connect to trampoline
    //}

    //pub fn connect_accel_changed<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    TODO: connect to trampoline
    //}
}
