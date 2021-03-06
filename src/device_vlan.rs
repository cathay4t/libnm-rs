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
    pub struct DeviceVlan(Object<nm_sys::NMDeviceVlan, nm_sys::NMDeviceVlanClass, DeviceVlanClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_vlan_get_type(),
    }
}

impl DeviceVlan {
    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// `true` if the device has carrier
    pub fn get_carrier(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_vlan_get_carrier(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's parent device
    pub fn get_parent(&self) -> Option<Device> {
        unsafe { from_glib_none(nm_sys::nm_device_vlan_get_parent(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's VLAN ID
    pub fn get_vlan_id(&self) -> u32 {
        unsafe { nm_sys::nm_device_vlan_get_vlan_id(self.to_glib_none().0) }
    }

    pub fn connect_property_carrier_notify<F: Fn(&DeviceVlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceVlan) + 'static>(
            this: *mut nm_sys::NMDeviceVlan,
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

    pub fn connect_property_parent_notify<F: Fn(&DeviceVlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&DeviceVlan) + 'static>(
            this: *mut nm_sys::NMDeviceVlan,
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
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_vlan_id_notify<F: Fn(&DeviceVlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_vlan_id_trampoline<F: Fn(&DeviceVlan) + 'static>(
            this: *mut nm_sys::NMDeviceVlan,
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
                b"notify::vlan-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vlan_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceVlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceVlan")
    }
}
