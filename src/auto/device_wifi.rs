// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use AccessPoint;
use Device;
use Error;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::StaticType;
use glib::Value;
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
    pub struct DeviceWifi(Object<ffi::NMDeviceWifi, ffi::NMDeviceWifiClass>): Device;

    match fn {
        get_type => || ffi::nm_device_wifi_get_type(),
    }
}

pub trait DeviceWifiExt {
    fn get_access_point_by_path(&self, path: &str) -> Option<AccessPoint>;

    //fn get_access_points(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 3 };

    fn get_active_access_point(&self) -> Option<AccessPoint>;

    fn get_bitrate(&self) -> u32;

    //fn get_capabilities(&self) -> /*Ignored*/DeviceWifiCapabilities;

    //fn get_mode(&self) -> /*Ignored*/80211Mode;

    fn get_permanent_hw_address(&self) -> Option<String>;

    fn request_scan<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn request_scan_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    //fn request_scan_options<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, options: /*Ignored*/&glib::Variant, cancellable: P) -> Result<(), Error>;

    //fn request_scan_options_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: /*Unimplemented*/gio::AsyncReadyCallback>(&self, options: /*Ignored*/&glib::Variant, cancellable: P, callback: Q);

    fn get_property_perm_hw_address(&self) -> Option<String>;

    //fn get_property_wireless_capabilities(&self) -> /*Ignored*/DeviceWifiCapabilities;

    fn connect_access_point_added<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_access_point_removed<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_access_points_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_access_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bitrate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_perm_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wireless_capabilities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceWifi> + IsA<glib::object::Object>> DeviceWifiExt for O {
    fn get_access_point_by_path(&self, path: &str) -> Option<AccessPoint> {
        unsafe {
            from_glib_none(ffi::nm_device_wifi_get_access_point_by_path(self.to_glib_none().0, path.to_glib_none().0))
        }
    }

    //fn get_access_points(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 3 } {
    //    unsafe { TODO: call ffi::nm_device_wifi_get_access_points() }
    //}

    fn get_active_access_point(&self) -> Option<AccessPoint> {
        unsafe {
            from_glib_none(ffi::nm_device_wifi_get_active_access_point(self.to_glib_none().0))
        }
    }

    fn get_bitrate(&self) -> u32 {
        unsafe {
            ffi::nm_device_wifi_get_bitrate(self.to_glib_none().0)
        }
    }

    //fn get_capabilities(&self) -> /*Ignored*/DeviceWifiCapabilities {
    //    unsafe { TODO: call ffi::nm_device_wifi_get_capabilities() }
    //}

    //fn get_mode(&self) -> /*Ignored*/80211Mode {
    //    unsafe { TODO: call ffi::nm_device_wifi_get_mode() }
    //}

    fn get_permanent_hw_address(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_device_wifi_get_permanent_hw_address(self.to_glib_none().0))
        }
    }

    fn request_scan<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_device_wifi_request_scan(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn request_scan_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn request_scan_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::nm_device_wifi_request_scan_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = request_scan_async_trampoline::<Q>;
        unsafe {
            ffi::nm_device_wifi_request_scan_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    //fn request_scan_options<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, options: /*Ignored*/&glib::Variant, cancellable: P) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::nm_device_wifi_request_scan_options() }
    //}

    //fn request_scan_options_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: /*Unimplemented*/gio::AsyncReadyCallback>(&self, options: /*Ignored*/&glib::Variant, cancellable: P, callback: Q) {
    //    unsafe { TODO: call ffi::nm_device_wifi_request_scan_options_async() }
    //}

    fn get_property_perm_hw_address(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "perm-hw-address".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    //fn get_property_wireless_capabilities(&self) -> /*Ignored*/DeviceWifiCapabilities {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "wireless-capabilities".to_glib_none().0, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    fn connect_access_point_added<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Object) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "access-point-added",
                transmute(access_point_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_access_point_removed<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Object) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "access-point-removed",
                transmute(access_point_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_access_points_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::access-points",
                transmute(notify_access_points_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_access_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active-access-point",
                transmute(notify_active_access_point_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_bitrate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bitrate",
                transmute(notify_bitrate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hw-address",
                transmute(notify_hw_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mode",
                transmute(notify_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_perm_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::perm-hw-address",
                transmute(notify_perm_hw_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wireless_capabilities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wireless-capabilities",
                transmute(notify_wireless_capabilities_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn access_point_added_trampoline<P>(this: *mut ffi::NMDeviceWifi, ap: *mut gobject_ffi::GObject, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P, &glib::Object) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(ap))
}

unsafe extern "C" fn access_point_removed_trampoline<P>(this: *mut ffi::NMDeviceWifi, ap: *mut gobject_ffi::GObject, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P, &glib::Object) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(ap))
}

unsafe extern "C" fn notify_access_points_trampoline<P>(this: *mut ffi::NMDeviceWifi, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_active_access_point_trampoline<P>(this: *mut ffi::NMDeviceWifi, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bitrate_trampoline<P>(this: *mut ffi::NMDeviceWifi, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hw_address_trampoline<P>(this: *mut ffi::NMDeviceWifi, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mode_trampoline<P>(this: *mut ffi::NMDeviceWifi, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_perm_hw_address_trampoline<P>(this: *mut ffi::NMDeviceWifi, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wireless_capabilities_trampoline<P>(this: *mut ffi::NMDeviceWifi, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceWifi> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceWifi::from_glib_borrow(this).downcast_unchecked())
}