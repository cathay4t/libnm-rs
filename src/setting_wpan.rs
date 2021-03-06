// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::object::Cast;
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

glib_wrapper! {
    pub struct SettingWpan(Object<nm_sys::NMSettingWpan, nm_sys::NMSettingWpanClass, SettingWpanClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_wpan_get_type(),
    }
}

impl SettingWpan {
    /// Creates a new `SettingWpan` object with default values.
    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the new empty `SettingWpan` object
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new() -> SettingWpan {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_wpan_new()).unsafe_cast() }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingWpan:channel` property of the setting
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_channel(&self) -> i16 {
        unsafe { nm_sys::nm_setting_wpan_get_channel(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingWpan:mac-address` property of the setting
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_mac_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wpan_get_mac_address(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingWpan:page` property of the setting
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_page(&self) -> i16 {
        unsafe { nm_sys::nm_setting_wpan_get_page(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingWpan:pan-id` property of the setting
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_pan_id(&self) -> u16 {
        unsafe { nm_sys::nm_setting_wpan_get_pan_id(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingWpan:short-address` property of the setting
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_short_address(&self) -> u16 {
        unsafe { nm_sys::nm_setting_wpan_get_short_address(self.to_glib_none().0) }
    }

    /// IEEE 802.15.4 channel. A positive integer or -1, meaning "do not
    /// set, use whatever the device is already set to".
    ///
    /// Feature: `v1_16`
    ///
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_channel(&self, channel: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"channel\0".as_ptr() as *const _,
                Value::from(&channel).to_glib_none().0,
            );
        }
    }

    /// If specified, this connection will only apply to the IEEE 802.15.4 (WPAN)
    /// MAC layer device whose permanent MAC address matches.
    pub fn get_property_mac_address(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"mac-address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `mac-address` getter")
        }
    }

    /// If specified, this connection will only apply to the IEEE 802.15.4 (WPAN)
    /// MAC layer device whose permanent MAC address matches.
    pub fn set_property_mac_address(&self, mac_address: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"mac-address\0".as_ptr() as *const _,
                Value::from(mac_address).to_glib_none().0,
            );
        }
    }

    /// IEEE 802.15.4 channel page. A positive integer or -1, meaning "do not
    /// set, use whatever the device is already set to".
    ///
    /// Feature: `v1_16`
    ///
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_page(&self, page: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"page\0".as_ptr() as *const _,
                Value::from(&page).to_glib_none().0,
            );
        }
    }

    /// IEEE 802.15.4 Personal Area Network (PAN) identifier.
    pub fn get_property_pan_id(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"pan-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pan-id` getter")
                .unwrap()
        }
    }

    /// IEEE 802.15.4 Personal Area Network (PAN) identifier.
    pub fn set_property_pan_id(&self, pan_id: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"pan-id\0".as_ptr() as *const _,
                Value::from(&pan_id).to_glib_none().0,
            );
        }
    }

    /// Short IEEE 802.15.4 address to be used within a restricted environment.
    pub fn get_property_short_address(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"short-address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `short-address` getter")
                .unwrap()
        }
    }

    /// Short IEEE 802.15.4 address to be used within a restricted environment.
    pub fn set_property_short_address(&self, short_address: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"short-address\0".as_ptr() as *const _,
                Value::from(&short_address).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_channel_notify<F: Fn(&SettingWpan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_channel_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut nm_sys::NMSettingWpan,
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
                b"notify::channel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_channel_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mac_address_notify<F: Fn(&SettingWpan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut nm_sys::NMSettingWpan,
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
                b"notify::mac-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mac_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_page_notify<F: Fn(&SettingWpan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut nm_sys::NMSettingWpan,
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
                b"notify::page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_pan_id_notify<F: Fn(&SettingWpan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pan_id_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut nm_sys::NMSettingWpan,
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
                b"notify::pan-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pan_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_short_address_notify<F: Fn(&SettingWpan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_short_address_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut nm_sys::NMSettingWpan,
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
                b"notify::short-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_short_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
impl Default for SettingWpan {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingWpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingWpan")
    }
}
