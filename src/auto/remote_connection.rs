// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Connection;
use Error;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RemoteConnection(Object<ffi::NMRemoteConnection, ffi::NMRemoteConnectionClass>): Connection;

    match fn {
        get_type => || ffi::nm_remote_connection_get_type(),
    }
}

pub trait RemoteConnectionExt {
    fn commit_changes<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, save_to_disk: bool, cancellable: P) -> Result<(), Error>;

    fn commit_changes_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, save_to_disk: bool, cancellable: P, callback: Q);

    fn delete<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn delete_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    fn get_secrets<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, setting_name: &str, cancellable: P) -> Result<glib::Variant, Error>;

    fn get_secrets_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<glib::Variant, Error>) + Send + 'static>(&self, setting_name: &str, cancellable: P, callback: Q);

    fn get_unsaved(&self) -> bool;

    fn get_visible(&self) -> bool;

    fn save<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn save_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    //#[cfg(any(feature = "v1_10_2", feature = "dox"))]
    //fn update2<'a, 'b, 'c, 'd, P: Into<Option<&'a glib::Variant>>, Q: Into<Option<&'b glib::Variant>>, R: Into<Option<&'c gio::Cancellable>>, S: Into<Option<&'d /*Unimplemented*/gio::AsyncReadyCallback>>, T: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, settings: P, flags: /*Ignored*/SettingsUpdate2Flags, args: Q, cancellable: R, callback: S, user_data: T);

    fn connect_property_unsaved_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RemoteConnection> + IsA<glib::object::Object>> RemoteConnectionExt for O {
    fn commit_changes<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, save_to_disk: bool, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_commit_changes(self.to_glib_none().0, save_to_disk.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn commit_changes_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, save_to_disk: bool, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn commit_changes_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_commit_changes_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = commit_changes_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_commit_changes_async(self.to_glib_none().0, save_to_disk.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn delete<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_delete(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn delete_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_delete_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = delete_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_delete_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn get_secrets<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, setting_name: &str, cancellable: P) -> Result<glib::Variant, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_remote_connection_get_secrets(self.to_glib_none().0, setting_name.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_secrets_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<glib::Variant, Error>) + Send + 'static>(&self, setting_name: &str, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn get_secrets_async_trampoline<Q: FnOnce(Result<glib::Variant, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let ret = ffi::nm_remote_connection_get_secrets_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_secrets_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_get_secrets_async(self.to_glib_none().0, setting_name.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn get_unsaved(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_remote_connection_get_unsaved(self.to_glib_none().0))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_remote_connection_get_visible(self.to_glib_none().0))
        }
    }

    fn save<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_save(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn save_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn save_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_save_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = save_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_save_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    //#[cfg(any(feature = "v1_10_2", feature = "dox"))]
    //fn update2<'a, 'b, 'c, 'd, P: Into<Option<&'a glib::Variant>>, Q: Into<Option<&'b glib::Variant>>, R: Into<Option<&'c gio::Cancellable>>, S: Into<Option<&'d /*Unimplemented*/gio::AsyncReadyCallback>>, T: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, settings: P, flags: /*Ignored*/SettingsUpdate2Flags, args: Q, cancellable: R, callback: S, user_data: T) {
    //    unsafe { TODO: call ffi::nm_remote_connection_update2() }
    //}

    fn connect_property_unsaved_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::unsaved",
                transmute(notify_unsaved_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible",
                transmute(notify_visible_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_unsaved_trampoline<P>(this: *mut ffi::NMRemoteConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RemoteConnection> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RemoteConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_trampoline<P>(this: *mut ffi::NMRemoteConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RemoteConnection> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RemoteConnection::from_glib_borrow(this).downcast_unchecked())
}
