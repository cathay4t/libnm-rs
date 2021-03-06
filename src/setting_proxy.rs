// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use crate::SettingProxyMethod;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use gobject_sys;
use nm_sys;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct SettingProxy(Object<nm_sys::NMSettingProxy, nm_sys::NMSettingProxyClass, SettingProxyClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_proxy_get_type(),
    }
}

impl SettingProxy {
    /// Creates a new `SettingProxy` object.
    ///
    /// Feature: `v1_6`
    ///
    ///
    /// # Returns
    ///
    /// the new empty `SettingProxy` object
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    pub fn new() -> SettingProxy {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_proxy_new()).unsafe_cast() }
    }
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
impl Default for SettingProxy {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_PROXY: Option<&SettingProxy> = None;

/// Trait containing all `SettingProxy` methods.
///
/// Feature: `v1_6`
///
/// # Implementors
///
/// [`SettingProxy`](struct.SettingProxy.html)
pub trait SettingProxyExt: 'static {
    ///
    /// Feature: `v1_6`
    ///
    ///
    /// # Returns
    ///
    /// `true` if this proxy configuration is only for browser
    /// clients/schemes, `false` otherwise.
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_browser_only(&self) -> bool;

    /// Returns the proxy configuration method. By default the value is `SettingProxyMethod::None`.
    /// `SettingProxyMethod::None` should be selected for a connection intended for direct network
    /// access.
    ///
    /// Feature: `v1_6`
    ///
    ///
    /// # Returns
    ///
    /// the proxy configuration method
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_method(&self) -> SettingProxyMethod;

    ///
    /// Feature: `v1_6`
    ///
    ///
    /// # Returns
    ///
    /// the PAC script
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_pac_script(&self) -> Option<GString>;

    ///
    /// Feature: `v1_6`
    ///
    ///
    /// # Returns
    ///
    /// the PAC URL for obtaining PAC file
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_pac_url(&self) -> Option<GString>;

    /// Whether the proxy configuration is for browser only.
    ///
    /// Feature: `v1_6`
    ///
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_browser_only(&self, browser_only: bool);

    /// Method for proxy configuration, Default is `SettingProxyMethod::None`
    ///
    /// Feature: `v1_6`
    ///
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_method(&self, method: i32);

    /// PAC script for the connection.
    ///
    /// Feature: `v1_6`
    ///
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_pac_script(&self, pac_script: Option<&str>);

    /// PAC URL for obtaining PAC file.
    ///
    /// Feature: `v1_6`
    ///
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_pac_url(&self, pac_url: Option<&str>);

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_browser_only_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_pac_script_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_pac_url_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingProxy>> SettingProxyExt for O {
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_browser_only(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_proxy_get_browser_only(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_method(&self) -> SettingProxyMethod {
        unsafe {
            from_glib(nm_sys::nm_setting_proxy_get_method(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_pac_script(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_proxy_get_pac_script(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_pac_url(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_proxy_get_pac_url(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_browser_only(&self, browser_only: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"browser-only\0".as_ptr() as *const _,
                Value::from(&browser_only).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_method(&self, method: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"method\0".as_ptr() as *const _,
                Value::from(&method).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_pac_script(&self, pac_script: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pac-script\0".as_ptr() as *const _,
                Value::from(pac_script).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn set_property_pac_url(&self, pac_url: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pac-url\0".as_ptr() as *const _,
                Value::from(pac_url).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_browser_only_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_browser_only_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingProxy,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingProxy>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::browser-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_browser_only_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_method_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingProxy,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingProxy>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::method\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_method_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_pac_script_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pac_script_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingProxy,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingProxy>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pac-script\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pac_script_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_pac_url_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pac_url_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingProxy,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingProxy>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pac-url\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pac_url_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingProxy")
    }
}
