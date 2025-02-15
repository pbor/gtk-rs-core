// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    AsyncResult, BufferedInputStream, Cancellable, DataStreamByteOrder, DataStreamNewlineType,
    FilterInputStream, InputStream, Seekable,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "GDataInputStream")]
    pub struct DataInputStream(Object<ffi::GDataInputStream, ffi::GDataInputStreamClass>) @extends BufferedInputStream, FilterInputStream, InputStream, @implements Seekable;

    match fn {
        type_ => || ffi::g_data_input_stream_get_type(),
    }
}

impl DataInputStream {
    pub const NONE: Option<&'static DataInputStream> = None;

    #[doc(alias = "g_data_input_stream_new")]
    pub fn new(base_stream: &impl IsA<InputStream>) -> DataInputStream {
        unsafe {
            from_glib_full(ffi::g_data_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DataInputStream`] objects.
    ///
    /// This method returns an instance of [`DataInputStreamBuilder`](crate::builders::DataInputStreamBuilder) which can be used to create [`DataInputStream`] objects.
    pub fn builder() -> DataInputStreamBuilder {
        DataInputStreamBuilder::default()
    }
}

impl Default for DataInputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DataInputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DataInputStreamBuilder {
    byte_order: Option<DataStreamByteOrder>,
    newline_type: Option<DataStreamNewlineType>,
    buffer_size: Option<u32>,
    base_stream: Option<InputStream>,
    close_base_stream: Option<bool>,
}

impl DataInputStreamBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`DataInputStreamBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DataInputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DataInputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref byte_order) = self.byte_order {
            properties.push(("byte-order", byte_order));
        }
        if let Some(ref newline_type) = self.newline_type {
            properties.push(("newline-type", newline_type));
        }
        if let Some(ref buffer_size) = self.buffer_size {
            properties.push(("buffer-size", buffer_size));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        glib::Object::new::<DataInputStream>(&properties)
    }

    pub fn byte_order(mut self, byte_order: DataStreamByteOrder) -> Self {
        self.byte_order = Some(byte_order);
        self
    }

    pub fn newline_type(mut self, newline_type: DataStreamNewlineType) -> Self {
        self.newline_type = Some(newline_type);
        self
    }

    pub fn buffer_size(mut self, buffer_size: u32) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn base_stream(mut self, base_stream: &impl IsA<InputStream>) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub trait DataInputStreamExt: 'static {
    #[doc(alias = "g_data_input_stream_get_byte_order")]
    #[doc(alias = "get_byte_order")]
    fn byte_order(&self) -> DataStreamByteOrder;

    #[doc(alias = "g_data_input_stream_get_newline_type")]
    #[doc(alias = "get_newline_type")]
    fn newline_type(&self) -> DataStreamNewlineType;

    #[doc(alias = "g_data_input_stream_read_byte")]
    fn read_byte(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u8, glib::Error>;

    #[doc(alias = "g_data_input_stream_read_int16")]
    fn read_int16(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i16, glib::Error>;

    #[doc(alias = "g_data_input_stream_read_int32")]
    fn read_int32(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i32, glib::Error>;

    #[doc(alias = "g_data_input_stream_read_int64")]
    fn read_int64(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i64, glib::Error>;

    #[doc(alias = "g_data_input_stream_read_uint16")]
    fn read_uint16(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u16, glib::Error>;

    #[doc(alias = "g_data_input_stream_read_uint32")]
    fn read_uint32(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u32, glib::Error>;

    #[doc(alias = "g_data_input_stream_read_uint64")]
    fn read_uint64(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u64, glib::Error>;

    #[doc(alias = "g_data_input_stream_set_byte_order")]
    fn set_byte_order(&self, order: DataStreamByteOrder);

    #[doc(alias = "g_data_input_stream_set_newline_type")]
    fn set_newline_type(&self, type_: DataStreamNewlineType);

    #[doc(alias = "byte-order")]
    fn connect_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "newline-type")]
    fn connect_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DataInputStream>> DataInputStreamExt for O {
    fn byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_byte_order(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn newline_type(&self) -> DataStreamNewlineType {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_newline_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn read_byte(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u8, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_byte(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_int16(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i16, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int16(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_int32(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int32(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_int64(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int64(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_uint16(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u16, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint16(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_uint32(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint32(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_uint64(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint64(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            ffi::g_data_input_stream_set_byte_order(
                self.as_ref().to_glib_none().0,
                order.into_glib(),
            );
        }
    }

    fn set_newline_type(&self, type_: DataStreamNewlineType) {
        unsafe {
            ffi::g_data_input_stream_set_newline_type(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            );
        }
    }

    fn connect_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_byte_order_trampoline<
            P: IsA<DataInputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDataInputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DataInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::byte-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_byte_order_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<
            P: IsA<DataInputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDataInputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DataInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_newline_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DataInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DataInputStream")
    }
}
