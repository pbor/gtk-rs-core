// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use crate::NetworkConnectivity;
use crate::SocketConnectable;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct NetworkMonitor(Interface<ffi::GNetworkMonitor, ffi::GNetworkMonitorInterface>);

    match fn {
        type_ => || ffi::g_network_monitor_get_type(),
    }
}

impl NetworkMonitor {
    #[doc(alias = "g_network_monitor_get_default")]
    pub fn default() -> NetworkMonitor {
        unsafe { from_glib_none(ffi::g_network_monitor_get_default()) }
    }
}

pub const NONE_NETWORK_MONITOR: Option<&NetworkMonitor> = None;

pub trait NetworkMonitorExt: 'static {
    #[doc(alias = "g_network_monitor_can_reach")]
    fn can_reach<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_network_monitor_can_reach_async")]
    fn can_reach_async<
        P: IsA<SocketConnectable>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn can_reach_async_future<P: IsA<SocketConnectable> + Clone + 'static>(
        &self,
        connectable: &P,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    #[doc(alias = "g_network_monitor_get_connectivity")]
    fn connectivity(&self) -> NetworkConnectivity;

    #[doc(alias = "g_network_monitor_get_network_available")]
    fn is_network_available(&self) -> bool;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
    #[doc(alias = "g_network_monitor_get_network_metered")]
    fn is_network_metered(&self) -> bool;

    fn connect_network_changed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn connect_property_connectivity_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_network_available_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
    fn connect_property_network_metered_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<NetworkMonitor>> NetworkMonitorExt for O {
    fn can_reach<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_network_monitor_can_reach(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn can_reach_async<
        P: IsA<SocketConnectable>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn can_reach_async_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::g_network_monitor_can_reach_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = can_reach_async_trampoline::<R>;
        unsafe {
            ffi::g_network_monitor_can_reach_async(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn can_reach_async_future<P: IsA<SocketConnectable> + Clone + 'static>(
        &self,
        connectable: &P,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let connectable = connectable.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.can_reach_async(&connectable, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn connectivity(&self) -> NetworkConnectivity {
        unsafe {
            from_glib(ffi::g_network_monitor_get_connectivity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_network_available(&self) -> bool {
        unsafe {
            from_glib(ffi::g_network_monitor_get_network_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
    fn is_network_metered(&self) -> bool {
        unsafe {
            from_glib(ffi::g_network_monitor_get_network_metered(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_network_changed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn network_changed_trampoline<P, F: Fn(&P, bool) + 'static>(
            this: *mut ffi::GNetworkMonitor,
            network_available: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NetworkMonitor>,
        {
            let f: &F = &*(f as *const F);
            f(
                &NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(network_available),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"network-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    network_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn connect_property_connectivity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_connectivity_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GNetworkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NetworkMonitor>,
        {
            let f: &F = &*(f as *const F);
            f(&NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::connectivity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_connectivity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_network_available_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_network_available_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GNetworkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NetworkMonitor>,
        {
            let f: &F = &*(f as *const F);
            f(&NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::network-available\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_network_available_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
    fn connect_property_network_metered_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_network_metered_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GNetworkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<NetworkMonitor>,
        {
            let f: &F = &*(f as *const F);
            f(&NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::network-metered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_network_metered_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for NetworkMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NetworkMonitor")
    }
}
