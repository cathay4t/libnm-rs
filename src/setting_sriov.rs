// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use gobject_sys;
use nm_sys;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use std::mem::transmute;
use Setting;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use SriovVF;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use Ternary;

glib_wrapper! {
    pub struct SettingSriov(Object<nm_sys::NMSettingSriov, nm_sys::NMSettingSriovClass, SettingSriovClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_sriov_get_type(),
    }
}

impl SettingSriov {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new() -> SettingSriov {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_sriov_new()).unsafe_cast() }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn add_vf(&self, vf: &SriovVF) {
        unsafe {
            nm_sys::nm_setting_sriov_add_vf(self.to_glib_none().0, vf.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn clear_vfs(&self) {
        unsafe {
            nm_sys::nm_setting_sriov_clear_vfs(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_autoprobe_drivers(&self) -> Ternary {
        unsafe {
            from_glib(nm_sys::nm_setting_sriov_get_autoprobe_drivers(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_num_vfs(&self) -> u32 {
        unsafe { nm_sys::nm_setting_sriov_get_num_vfs(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_total_vfs(&self) -> u32 {
        unsafe { nm_sys::nm_setting_sriov_get_total_vfs(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_vf(&self, idx: u32) -> Option<SriovVF> {
        unsafe { from_glib_none(nm_sys::nm_setting_sriov_get_vf(self.to_glib_none().0, idx)) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn remove_vf(&self, idx: u32) {
        unsafe {
            nm_sys::nm_setting_sriov_remove_vf(self.to_glib_none().0, idx);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn remove_vf_by_index(&self, index: u32) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_sriov_remove_vf_by_index(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_property_autoprobe_drivers(&self, autoprobe_drivers: Ternary) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"autoprobe-drivers\0".as_ptr() as *const _,
                Value::from(&autoprobe_drivers).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_property_total_vfs(&self, total_vfs: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"total-vfs\0".as_ptr() as *const _,
                Value::from(&total_vfs).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn connect_property_autoprobe_drivers_notify<F: Fn(&SettingSriov) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoprobe_drivers_trampoline<F: Fn(&SettingSriov) + 'static>(
            this: *mut nm_sys::NMSettingSriov,
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
                b"notify::autoprobe-drivers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autoprobe_drivers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn connect_property_total_vfs_notify<F: Fn(&SettingSriov) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_total_vfs_trampoline<F: Fn(&SettingSriov) + 'static>(
            this: *mut nm_sys::NMSettingSriov,
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
                b"notify::total-vfs\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_total_vfs_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
impl Default for SettingSriov {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingSriov {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingSriov")
    }
}