// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{prelude::*, SocketControlMessage};

pub trait SocketControlMessageExtManual: Sized {
    #[doc(alias = "g_socket_control_message_serialize")]
    fn serialize(&self, data: &mut [u8]);
}

impl<O: IsA<SocketControlMessage>> SocketControlMessageExtManual for O {
    #[doc(alias = "g_socket_control_message_serialize")]
    fn serialize(&self, data: &mut [u8]) {
        assert!(data.len() >= self.size());
        unsafe {
            ffi::g_socket_control_message_serialize(
                self.as_ref().to_glib_none().0,
                data.as_mut_ptr() as *mut _,
            );
        }
    }
}
