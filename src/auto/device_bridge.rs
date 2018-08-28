// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Device;

glib_wrapper! {
    pub struct DeviceBridge(Object<ffi::NMDeviceBridge, ffi::NMDeviceBridgeClass>): Device;

    match fn {
        get_type => || ffi::nm_device_bridge_get_type(),
    }
}

pub trait DeviceBridgeExt {
    fn get_carrier(&self) -> bool;

    //fn get_slaves(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 11 };

    fn connect_property_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_slaves_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceBridge> + IsA<glib::object::Object>> DeviceBridgeExt for O {
    fn get_carrier(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_bridge_get_carrier(self.to_glib_none().0)) }
    }

    //fn get_slaves(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 11 } {
    //    unsafe { TODO: call ffi::nm_device_bridge_get_slaves() }
    //}

    fn connect_property_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::carrier",
                transmute(notify_carrier_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::hw-address",
                transmute(notify_hw_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_slaves_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::slaves",
                transmute(notify_slaves_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_carrier_trampoline<P>(
    this: *mut ffi::NMDeviceBridge,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceBridge>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceBridge::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hw_address_trampoline<P>(
    this: *mut ffi::NMDeviceBridge,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceBridge>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceBridge::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_slaves_trampoline<P>(
    this: *mut ffi::NMDeviceBridge,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceBridge>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceBridge::from_glib_borrow(this).downcast_unchecked())
}
