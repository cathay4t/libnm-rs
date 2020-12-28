use crate::Connection;
use crate::KeyfileHandlerFlags;
use glib;

pub fn keyfile_write(
    connection: &Connection,
    handler_flags: KeyfileHandlerFlags,
) -> Result<glib::GKeyFile, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let key_file = nm_sys::nm_keyfile_write(
            connection.to_glib_none().0,
            handler_flags.to_glib(),
            ptr::null(),
            ptr::null(),
            &mut error,
        );
        if error.is_null() {
            Ok(glib::translate::from_glib_full(key_file))
        } else {
            Err(from_glib_full(error))
        }
    }
}
