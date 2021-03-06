// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use crate::SriovVF;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use crate::Ternary;
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

glib_wrapper! {
    pub struct SettingSriov(Object<nm_sys::NMSettingSriov, nm_sys::NMSettingSriovClass, SettingSriovClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_sriov_get_type(),
    }
}

impl SettingSriov {
    /// Creates a new `SettingSriov` object with default values.
    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the new empty `SettingSriov` object
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new() -> SettingSriov {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_sriov_new()).unsafe_cast() }
    }

    /// Appends a new VF and associated information to the setting. The
    /// given VF is duplicated internally and is not changed by this function.
    ///
    /// Feature: `v1_14`
    ///
    /// ## `vf`
    /// the VF to add
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn add_vf(&self, vf: &SriovVF) {
        unsafe {
            nm_sys::nm_setting_sriov_add_vf(self.to_glib_none().0, vf.to_glib_none().0);
        }
    }

    /// Removes all configured VFs.
    ///
    /// Feature: `v1_14`
    ///
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn clear_vfs(&self) {
        unsafe {
            nm_sys::nm_setting_sriov_clear_vfs(self.to_glib_none().0);
        }
    }

    /// Returns the value contained in the `SettingSriov:autoprobe-drivers`
    /// property.
    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the autoprobe-drivers property value
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_autoprobe_drivers(&self) -> Ternary {
        unsafe {
            from_glib(nm_sys::nm_setting_sriov_get_autoprobe_drivers(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the number of configured VFs
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_num_vfs(&self) -> u32 {
        unsafe { nm_sys::nm_setting_sriov_get_num_vfs(self.to_glib_none().0) }
    }

    /// Returns the value contained in the `SettingSriov:total-vfs`
    /// property.
    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the total number of SR-IOV virtual functions to create
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_total_vfs(&self) -> u32 {
        unsafe { nm_sys::nm_setting_sriov_get_total_vfs(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_14`
    ///
    /// ## `idx`
    /// index number of the VF to return
    ///
    /// # Returns
    ///
    /// the VF at index `idx`
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_vf(&self, idx: u32) -> Option<SriovVF> {
        unsafe { from_glib_none(nm_sys::nm_setting_sriov_get_vf(self.to_glib_none().0, idx)) }
    }

    /// Removes the VF at index `idx`.
    ///
    /// Feature: `v1_14`
    ///
    /// ## `idx`
    /// index number of the VF
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn remove_vf(&self, idx: u32) {
        unsafe {
            nm_sys::nm_setting_sriov_remove_vf(self.to_glib_none().0, idx);
        }
    }

    /// Removes the VF with VF index `index`.
    ///
    /// Feature: `v1_14`
    ///
    /// ## `index`
    /// the VF index of the VF to remove
    ///
    /// # Returns
    ///
    /// `true` if the VF was found and removed; `false` if it was not
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn remove_vf_by_index(&self, index: u32) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_sriov_remove_vf_by_index(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    /// Whether to autoprobe virtual functions by a compatible driver.
    ///
    /// If set to `Ternary::True`, the kernel will try to bind VFs to
    /// a compatible driver and if this succeeds a new network
    /// interface will be instantiated for each VF.
    ///
    /// If set to `Ternary::False`, VFs will not be claimed and no
    /// network interfaces will be created for them.
    ///
    /// When set to `Ternary::Default`, the global default is used; in
    /// case the global default is unspecified it is assumed to be
    /// `Ternary::True`.
    ///
    /// Feature: `v1_14`
    ///
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

    /// The total number of virtual functions to create.
    ///
    /// Note that when the sriov setting is present NetworkManager
    /// enforces the number of virtual functions on the interface
    /// (also when it is zero) during activation and resets it
    /// upon deactivation. To prevent any changes to SR-IOV
    /// parameters don't add a sriov setting to the connection.
    ///
    /// Feature: `v1_14`
    ///
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
