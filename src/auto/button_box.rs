// This file was generated by gir (b798f4f) from gir-files (11e0e6d)
// DO NOT EDIT

use Box;
use ButtonBoxStyle;
use Container;
use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ButtonBox(Object<ffi::GtkButtonBox>): Box, Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_button_box_get_type(),
    }
}

impl ButtonBox {
    pub fn new(orientation: Orientation) -> ButtonBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_box_new(orientation.to_glib())).downcast_unchecked()
        }
    }

    pub fn get_child_non_homogeneous<T: IsA<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_non_homogeneous(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_child_secondary<T: IsA<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_secondary(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_layout(&self) -> ButtonBoxStyle {
        unsafe {
            from_glib(ffi::gtk_button_box_get_layout(self.to_glib_none().0))
        }
    }

    pub fn set_child_non_homogeneous<T: IsA<Widget>>(&self, child: &T, non_homogeneous: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_non_homogeneous(self.to_glib_none().0, child.to_glib_none().0, non_homogeneous.to_glib());
        }
    }

    pub fn set_child_secondary<T: IsA<Widget>>(&self, child: &T, is_secondary: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_secondary(self.to_glib_none().0, child.to_glib_none().0, is_secondary.to_glib());
        }
    }

    pub fn set_layout(&self, layout_style: ButtonBoxStyle) {
        unsafe {
            ffi::gtk_button_box_set_layout(self.to_glib_none().0, layout_style.to_glib());
        }
    }
}
