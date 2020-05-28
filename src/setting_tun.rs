// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use gobject_sys;
use nm_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::mem::transmute;
use Setting;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use SettingTunMode;

glib_wrapper! {
    pub struct SettingTun(Object<nm_sys::NMSettingTun, nm_sys::NMSettingTunClass, SettingTunClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_tun_get_type(),
    }
}

impl SettingTun {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn new() -> SettingTun {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_tun_new()).unsafe_cast() }
    }
}

#[cfg(any(feature = "v1_2", feature = "dox"))]
impl Default for SettingTun {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_TUN: Option<&SettingTun> = None;

pub trait SettingTunExt: 'static {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_group(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_mode(&self) -> SettingTunMode;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_multi_queue(&self) -> bool;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_owner(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_pi(&self) -> bool;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_vnet_hdr(&self) -> bool;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_group(&self, group: Option<&str>);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_mode(&self, mode: u32);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_multi_queue(&self, multi_queue: bool);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_owner(&self, owner: Option<&str>);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_pi(&self, pi: bool);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_vnet_hdr(&self, vnet_hdr: bool);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_multi_queue_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_pi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_vnet_hdr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingTun>> SettingTunExt for O {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_group(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_tun_get_group(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_mode(&self) -> SettingTunMode {
        unsafe {
            from_glib(nm_sys::nm_setting_tun_get_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_multi_queue(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_tun_get_multi_queue(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_owner(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_tun_get_owner(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_pi(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_tun_get_pi(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_vnet_hdr(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_tun_get_vnet_hdr(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_group(&self, group: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"group\0".as_ptr() as *const _,
                Value::from(group).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_mode(&self, mode: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mode\0".as_ptr() as *const _,
                Value::from(&mode).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_multi_queue(&self, multi_queue: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"multi-queue\0".as_ptr() as *const _,
                Value::from(&multi_queue).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_owner(&self, owner: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"owner\0".as_ptr() as *const _,
                Value::from(owner).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_pi(&self, pi: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pi\0".as_ptr() as *const _,
                Value::from(&pi).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_vnet_hdr(&self, vnet_hdr: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"vnet-hdr\0".as_ptr() as *const _,
                Value::from(&vnet_hdr).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingTun,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingTun>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingTun::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_group_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingTun,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingTun>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingTun::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_multi_queue_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_multi_queue_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingTun,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingTun>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingTun::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multi-queue\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multi_queue_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingTun,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingTun>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingTun::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::owner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_owner_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_pi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pi_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingTun,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingTun>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingTun::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pi\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pi_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_vnet_hdr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vnet_hdr_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingTun,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingTun>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingTun::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vnet-hdr\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vnet_hdr_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingTun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingTun")
    }
}