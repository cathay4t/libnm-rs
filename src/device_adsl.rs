// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Device;
use crate::Object;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DeviceAdsl(Object<nm_sys::NMDeviceAdsl, nm_sys::NMDeviceAdslClass, DeviceAdslClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_adsl_get_type(),
    }
}

impl DeviceAdsl {
    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// `true` if the device has carrier
    pub fn get_carrier(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_adsl_get_carrier(self.to_glib_none().0)) }
    }

    pub fn connect_property_carrier_notify<F: Fn(&DeviceAdsl) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceAdsl) + 'static>(
            this: *mut nm_sys::NMDeviceAdsl,
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
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceAdsl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceAdsl")
    }
}
