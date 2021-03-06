// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AccessPoint;
use crate::Device;
use crate::DeviceWifiCapabilities;
use crate::Object;
use crate::_80211Mode;
use gio;
use gio_sys;
use glib;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib_wrapper! {
    pub struct DeviceWifi(Object<nm_sys::NMDeviceWifi, nm_sys::NMDeviceWifiClass, DeviceWifiClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_wifi_get_type(),
    }
}

impl DeviceWifi {
    /// Gets a `AccessPoint` by path.
    /// ## `path`
    /// the object path of the access point
    ///
    /// # Returns
    ///
    /// the access point or `None` if none is found.
    pub fn get_access_point_by_path(&self, path: &str) -> Option<AccessPoint> {
        unsafe {
            from_glib_none(nm_sys::nm_device_wifi_get_access_point_by_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    /// Gets all the scanned access points of the `DeviceWifi`.
    ///
    /// # Returns
    ///
    /// a `glib::PtrArray` containing all the
    /// scanned `NMAccessPoints`.
    /// The returned array is owned by the client and should not be modified.
    pub fn get_access_points(&self) -> Vec<AccessPoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_device_wifi_get_access_points(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the active `AccessPoint`.
    ///
    /// # Returns
    ///
    /// the access point or `None` if none is active
    pub fn get_active_access_point(&self) -> Option<AccessPoint> {
        unsafe {
            from_glib_none(nm_sys::nm_device_wifi_get_active_access_point(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the bit rate of the `DeviceWifi` in kbit/s.
    ///
    /// # Returns
    ///
    /// the bit rate (kbit/s)
    pub fn get_bitrate(&self) -> u32 {
        unsafe { nm_sys::nm_device_wifi_get_bitrate(self.to_glib_none().0) }
    }

    /// Gets the Wi-Fi capabilities of the `DeviceWifi`.
    ///
    /// # Returns
    ///
    /// the capabilities
    pub fn get_capabilities(&self) -> DeviceWifiCapabilities {
        unsafe {
            from_glib(nm_sys::nm_device_wifi_get_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    /// Returns the timestamp (in CLOCK_BOOTTIME milliseconds) for the last finished
    /// network scan. A value of -1 means the device never scanned for access points.
    ///
    /// Use `nm_utils_get_timestamp_msec` to obtain current time value suitable for
    /// comparing to this value.
    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the last scan time in seconds
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_last_scan(&self) -> i64 {
        unsafe { nm_sys::nm_device_wifi_get_last_scan(self.to_glib_none().0) }
    }

    /// Gets the `DeviceWifi` mode.
    ///
    /// # Returns
    ///
    /// the mode
    pub fn get_mode(&self) -> _80211Mode {
        unsafe { from_glib(nm_sys::nm_device_wifi_get_mode(self.to_glib_none().0)) }
    }

    /// Gets the permanent hardware (MAC) address of the `DeviceWifi`
    ///
    /// # Returns
    ///
    /// the permanent hardware address. This is the internal string used by the
    /// device, and must not be modified.
    pub fn get_permanent_hw_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_device_wifi_get_permanent_hw_address(
                self.to_glib_none().0,
            ))
        }
    }

    /// Request NM to scan for access points on `self`. Note that the function
    /// returns immediately after requesting the scan, and it may take some time
    /// after that for the scan to complete.
    ///
    /// # Deprecated since 1.22
    ///
    /// Use `DeviceWifi::request_scan_async` or GDBusConnection.
    /// ## `cancellable`
    /// a `gio::Cancellable`, or `None`
    ///
    /// # Returns
    ///
    /// `true` on success, `false` on error, in which case `error` will be
    /// set.
    #[cfg_attr(feature = "v1_22", deprecated)]
    pub fn request_scan<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_device_wifi_request_scan(
                self.to_glib_none().0,
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

    /// Request NM to scan for access points on `self`. Note that `callback` will be
    /// called immediately after requesting the scan, and it may take some time after
    /// that for the scan to complete.
    /// ## `cancellable`
    /// a `gio::Cancellable`, or `None`
    /// ## `callback`
    /// callback to be called when the scan has been requested
    /// ## `user_data`
    /// caller-specific data passed to `callback`
    pub fn request_scan_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn request_scan_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_device_wifi_request_scan_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = request_scan_async_trampoline::<Q>;
        unsafe {
            nm_sys::nm_device_wifi_request_scan_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn request_scan_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.request_scan_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    /// Request NM to scan for access points on `self`. Note that the function
    /// returns immediately after requesting the scan, and it may take some time
    /// after that for the scan to complete.
    /// This is the same as `DeviceWifi::request_scan` except it accepts `options`
    /// for the scanning. The argument is the dictionary passed to RequestScan()
    /// D-Bus call. Valid options inside the dictionary are:
    /// 'ssids' => array of SSIDs (saay)
    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Deprecated since 1.22
    ///
    /// Use `DeviceWifi::request_scan_options_async` or GDBusConnection.
    /// ## `options`
    /// dictionary with options for RequestScan(), or `None`
    /// ## `cancellable`
    /// a `gio::Cancellable`, or `None`
    ///
    /// # Returns
    ///
    /// `true` on success, `false` on error, in which case `error` will be
    /// set.
    #[cfg_attr(feature = "v1_22", deprecated)]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn request_scan_options<P: IsA<gio::Cancellable>>(
        &self,
        options: &glib::Variant,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_device_wifi_request_scan_options(
                self.to_glib_none().0,
                options.to_glib_none().0,
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

    //#[cfg(any(feature = "v1_2", feature = "dox"))]
    //pub fn request_scan_options_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, options: &glib::Variant, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call nm_sys:nm_device_wifi_request_scan_options_async() }
    //}

    /// The hardware (MAC) address of the device.
    pub fn get_property_perm_hw_address(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"perm-hw-address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `perm-hw-address` getter")
        }
    }

    /// The wireless capabilities of the device.
    pub fn get_property_wireless_capabilities(&self) -> DeviceWifiCapabilities {
        unsafe {
            let mut value = Value::from_type(<DeviceWifiCapabilities as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"wireless-capabilities\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `wireless-capabilities` getter")
                .unwrap()
        }
    }

    /// Notifies that a `AccessPoint` is added to the Wi-Fi device.
    /// ## `ap`
    /// the new access point
    pub fn connect_access_point_added<F: Fn(&DeviceWifi, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn access_point_added_trampoline<
            F: Fn(&DeviceWifi, &glib::Object) + 'static,
        >(
            this: *mut nm_sys::NMDeviceWifi,
            ap: *mut gobject_sys::GObject,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(ap))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"access-point-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    access_point_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// Notifies that a `AccessPoint` is removed from the Wi-Fi device.
    /// ## `ap`
    /// the removed access point
    pub fn connect_access_point_removed<F: Fn(&DeviceWifi, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn access_point_removed_trampoline<
            F: Fn(&DeviceWifi, &glib::Object) + 'static,
        >(
            this: *mut nm_sys::NMDeviceWifi,
            ap: *mut gobject_sys::GObject,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(ap))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"access-point-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    access_point_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_access_points_notify<F: Fn(&DeviceWifi) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_access_points_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut nm_sys::NMDeviceWifi,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::access-points\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_access_points_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_active_access_point_notify<F: Fn(&DeviceWifi) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_access_point_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut nm_sys::NMDeviceWifi,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-access-point\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_access_point_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_bitrate_notify<F: Fn(&DeviceWifi) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bitrate_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut nm_sys::NMDeviceWifi,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bitrate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bitrate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn connect_property_last_scan_notify<F: Fn(&DeviceWifi) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_scan_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut nm_sys::NMDeviceWifi,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::last-scan\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_last_scan_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mode_notify<F: Fn(&DeviceWifi) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut nm_sys::NMDeviceWifi,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_perm_hw_address_notify<F: Fn(&DeviceWifi) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_perm_hw_address_trampoline<F: Fn(&DeviceWifi) + 'static>(
            this: *mut nm_sys::NMDeviceWifi,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::perm-hw-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_perm_hw_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_wireless_capabilities_notify<F: Fn(&DeviceWifi) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wireless_capabilities_trampoline<
            F: Fn(&DeviceWifi) + 'static,
        >(
            this: *mut nm_sys::NMDeviceWifi,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wireless-capabilities\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wireless_capabilities_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceWifi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceWifi")
    }
}
