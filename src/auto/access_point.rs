// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use 80211Mode;
use Connection;
use ffi;
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
    pub struct AccessPoint(Object<ffi::NMAccessPoint, ffi::NMAccessPointClass>);

    match fn {
        get_type => || ffi::nm_access_point_get_type(),
    }
}

pub trait AccessPointExt {
    fn connection_valid<P: IsA<Connection>>(&self, connection: &P) -> bool;

    //fn filter_connections(&self, connections: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 4 }) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 4 };

    fn get_bssid(&self) -> Option<String>;

    //fn get_flags(&self) -> /*Ignored*/80211ApFlags;

    fn get_frequency(&self) -> u32;

    fn get_last_seen(&self) -> i32;

    fn get_max_bitrate(&self) -> u32;

    fn get_mode(&self) -> 80211Mode;

    //fn get_rsn_flags(&self) -> /*Ignored*/80211ApSecurityFlags;

    fn get_ssid(&self) -> Option<glib::Bytes>;

    fn get_strength(&self) -> u8;

    //fn get_wpa_flags(&self) -> /*Ignored*/80211ApSecurityFlags;

    #[deprecated]
    fn get_property_hw_address(&self) -> Option<String>;

    fn connect_property_bssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_frequency_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated]
    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_last_seen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_bitrate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rsn_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_strength_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wpa_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AccessPoint> + IsA<glib::object::Object>> AccessPointExt for O {
    fn connection_valid<P: IsA<Connection>>(&self, connection: &P) -> bool {
        unsafe {
            from_glib(ffi::nm_access_point_connection_valid(self.to_glib_none().0, connection.to_glib_none().0))
        }
    }

    //fn filter_connections(&self, connections: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 4 }) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 4 } {
    //    unsafe { TODO: call ffi::nm_access_point_filter_connections() }
    //}

    fn get_bssid(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_access_point_get_bssid(self.to_glib_none().0))
        }
    }

    //fn get_flags(&self) -> /*Ignored*/80211ApFlags {
    //    unsafe { TODO: call ffi::nm_access_point_get_flags() }
    //}

    fn get_frequency(&self) -> u32 {
        unsafe {
            ffi::nm_access_point_get_frequency(self.to_glib_none().0)
        }
    }

    fn get_last_seen(&self) -> i32 {
        unsafe {
            ffi::nm_access_point_get_last_seen(self.to_glib_none().0)
        }
    }

    fn get_max_bitrate(&self) -> u32 {
        unsafe {
            ffi::nm_access_point_get_max_bitrate(self.to_glib_none().0)
        }
    }

    fn get_mode(&self) -> 80211Mode {
        unsafe {
            from_glib(ffi::nm_access_point_get_mode(self.to_glib_none().0))
        }
    }

    //fn get_rsn_flags(&self) -> /*Ignored*/80211ApSecurityFlags {
    //    unsafe { TODO: call ffi::nm_access_point_get_rsn_flags() }
    //}

    fn get_ssid(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(ffi::nm_access_point_get_ssid(self.to_glib_none().0))
        }
    }

    fn get_strength(&self) -> u8 {
        unsafe {
            ffi::nm_access_point_get_strength(self.to_glib_none().0)
        }
    }

    //fn get_wpa_flags(&self) -> /*Ignored*/80211ApSecurityFlags {
    //    unsafe { TODO: call ffi::nm_access_point_get_wpa_flags() }
    //}

    fn get_property_hw_address(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "hw-address".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_bssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bssid",
                transmute(notify_bssid_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flags",
                transmute(notify_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_frequency_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::frequency",
                transmute(notify_frequency_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hw-address",
                transmute(notify_hw_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_last_seen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::last-seen",
                transmute(notify_last_seen_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_bitrate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-bitrate",
                transmute(notify_max_bitrate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mode",
                transmute(notify_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rsn_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rsn-flags",
                transmute(notify_rsn_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ssid",
                transmute(notify_ssid_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_strength_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::strength",
                transmute(notify_strength_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wpa_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wpa-flags",
                transmute(notify_wpa_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_bssid_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_flags_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_frequency_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hw_address_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_last_seen_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_max_bitrate_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mode_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rsn_flags_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ssid_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_strength_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wpa_flags_trampoline<P>(this: *mut ffi::NMAccessPoint, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AccessPoint> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AccessPoint::from_glib_borrow(this).downcast_unchecked())
}
