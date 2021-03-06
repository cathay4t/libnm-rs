// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Connection;
use crate::SettingCompareFlags;
use crate::SettingSecretFlags;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_26", feature = "dox"))]
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Setting(Object<nm_sys::NMSetting, nm_sys::NMSettingClass, SettingClass>);

    match fn {
        get_type => || nm_sys::nm_setting_get_type(),
    }
}

impl Setting {
    /// Returns the `glib::Type` of the setting's class for a given setting name.
    /// ## `name`
    /// a setting name
    ///
    /// # Returns
    ///
    /// the `glib::Type` of the setting's class, or `G_TYPE_INVALID` if
    ///  `name` is not recognized.
    pub fn lookup_type(name: &str) -> glib::types::Type {
        unsafe { from_glib(nm_sys::nm_setting_lookup_type(name.to_glib_none().0)) }
    }
}

pub const NONE_SETTING: Option<&Setting> = None;

/// Trait containing all `Setting` methods.
///
/// # Implementors
///
/// [`Setting6Lowpan`](struct.Setting6Lowpan.html), [`Setting8021x`](struct.Setting8021x.html), [`SettingAdsl`](struct.SettingAdsl.html), [`SettingBluetooth`](struct.SettingBluetooth.html), [`SettingBond`](struct.SettingBond.html), [`SettingBridgePort`](struct.SettingBridgePort.html), [`SettingBridge`](struct.SettingBridge.html), [`SettingCdma`](struct.SettingCdma.html), [`SettingConnection`](struct.SettingConnection.html), [`SettingDcb`](struct.SettingDcb.html), [`SettingDummy`](struct.SettingDummy.html), [`SettingEthtool`](struct.SettingEthtool.html), [`SettingGeneric`](struct.SettingGeneric.html), [`SettingGsm`](struct.SettingGsm.html), [`SettingIPConfig`](struct.SettingIPConfig.html), [`SettingIPTunnel`](struct.SettingIPTunnel.html), [`SettingInfiniband`](struct.SettingInfiniband.html), [`SettingMacsec`](struct.SettingMacsec.html), [`SettingMacvlan`](struct.SettingMacvlan.html), [`SettingMatch`](struct.SettingMatch.html), [`SettingOlpcMesh`](struct.SettingOlpcMesh.html), [`SettingOvsBridge`](struct.SettingOvsBridge.html), [`SettingOvsDpdk`](struct.SettingOvsDpdk.html), [`SettingOvsInterface`](struct.SettingOvsInterface.html), [`SettingOvsPatch`](struct.SettingOvsPatch.html), [`SettingOvsPort`](struct.SettingOvsPort.html), [`SettingPpp`](struct.SettingPpp.html), [`SettingPppoe`](struct.SettingPppoe.html), [`SettingProxy`](struct.SettingProxy.html), [`SettingSerial`](struct.SettingSerial.html), [`SettingSriov`](struct.SettingSriov.html), [`SettingTCConfig`](struct.SettingTCConfig.html), [`SettingTeamPort`](struct.SettingTeamPort.html), [`SettingTeam`](struct.SettingTeam.html), [`SettingTun`](struct.SettingTun.html), [`SettingUser`](struct.SettingUser.html), [`SettingVlan`](struct.SettingVlan.html), [`SettingVpn`](struct.SettingVpn.html), [`SettingVrf`](struct.SettingVrf.html), [`SettingVxlan`](struct.SettingVxlan.html), [`SettingWifiP2P`](struct.SettingWifiP2P.html), [`SettingWimax`](struct.SettingWimax.html), [`SettingWireGuard`](struct.SettingWireGuard.html), [`SettingWired`](struct.SettingWired.html), [`SettingWirelessSecurity`](struct.SettingWirelessSecurity.html), [`SettingWireless`](struct.SettingWireless.html), [`SettingWpan`](struct.SettingWpan.html), [`Setting`](struct.Setting.html)
pub trait SettingExt: 'static {
    /// Compares two `Setting` objects for similarity, with comparison behavior
    /// modified by a set of flags. See the documentation for `SettingCompareFlags`
    /// for a description of each flag's behavior.
    /// ## `b`
    /// a second `Setting` to compare with the first
    /// ## `flags`
    /// compare flags, e.g. `SettingCompareFlags::Exact`
    ///
    /// # Returns
    ///
    /// `true` if the comparison succeeds, `false` if it does not
    fn compare<P: IsA<Setting>>(&self, b: &P, flags: SettingCompareFlags) -> bool;

    //fn diff<P: IsA<Setting>>(&self, b: &P, flags: SettingCompareFlags, invert_results: bool, results: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 7 }) -> bool;

    /// Duplicates a `Setting`.
    ///
    /// # Returns
    ///
    /// a new `Setting` containing the same properties and values as the
    /// source `Setting`
    fn duplicate(&self) -> Option<Setting>;

    //fn enumerate_values(&self, func: /*Unimplemented*/FnMut(&Setting, &str, /*Ignored*/glib::Value, /*Ignored*/glib::ParamFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    /// Gets the D-Bus marshalling type of a property. `property_name` is a D-Bus
    /// property name, which may not necessarily be a `gobject::Object` property.
    /// ## `property_name`
    /// the property of `self` to get the type of
    ///
    /// # Returns
    ///
    /// the D-Bus marshalling type of `property` on `self`.
    fn get_dbus_property_type(&self, property_name: &str) -> Option<glib::VariantType>;

    /// Returns the type name of the `Setting` object
    ///
    /// # Returns
    ///
    /// a string containing the type name of the `Setting` object,
    /// like 'ppp' or 'wireless' or 'wired'.
    fn get_name(&self) -> Option<GString>;

    //#[cfg(any(feature = "v1_26", feature = "dox"))]
    //fn option_clear_by_name(&self, predicate: Option<&mut dyn (FnMut(&str) -> bool)>);

    ///
    /// Feature: `v1_26`
    ///
    /// ## `opt_name`
    /// the option name to request.
    ///
    /// # Returns
    ///
    /// the `glib::Variant` or `None` if the option
    ///  is not set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get(&self, opt_name: &str) -> Option<glib::Variant>;

    /// Gives the name of all set options.
    ///
    /// Feature: `v1_26`
    ///
    ///
    /// # Returns
    ///
    ///
    ///  A `None` terminated array of key names. If no names are present, this returns
    ///  `None`. The returned array and the names are owned by `Setting` and might be invalidated
    ///  by the next operation.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get_all_names(&self) -> Vec<GString>;

    ///
    /// Feature: `v1_26`
    ///
    /// ## `opt_name`
    /// the option to get
    /// ## `out_value`
    /// the optional output value.
    ///  If the option is unset, `false` will be returned.
    ///
    /// # Returns
    ///
    /// `true` if `opt_name` is set to a boolean variant.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get_boolean(&self, opt_name: &str) -> Option<bool>;

    ///
    /// Feature: `v1_26`
    ///
    /// ## `opt_name`
    /// the option to get
    /// ## `out_value`
    /// the optional output value.
    ///  If the option is unset, 0 will be returned.
    ///
    /// # Returns
    ///
    /// `true` if `opt_name` is set to a uint32 variant.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get_uint32(&self, opt_name: &str) -> Option<u32>;

    /// If `variant` is `None`, this clears the option if it is set.
    /// Otherwise, `variant` is set as the option. If `variant` is
    /// a floating reference, it will be consumed.
    ///
    /// Note that not all setting types support options. It is a bug
    /// setting a variant to a setting that doesn't support it.
    /// Currently only `SettingEthtool` supports it.
    ///
    /// Feature: `v1_26`
    ///
    /// ## `opt_name`
    /// the option name to set
    /// ## `variant`
    /// the variant to set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_set(&self, opt_name: &str, variant: Option<&glib::Variant>);

    /// Like `SettingExt::option_set` to set a boolean GVariant.
    ///
    /// Feature: `v1_26`
    ///
    /// ## `value`
    /// the value to set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_set_boolean(&self, opt_name: &str, value: bool);

    /// Like `SettingExt::option_set` to set a uint32 GVariant.
    ///
    /// Feature: `v1_26`
    ///
    /// ## `value`
    /// the value to set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_set_uint32(&self, opt_name: &str, value: u32);

    /// For a given secret, stores the `SettingSecretFlags` describing how to
    /// handle that secret.
    /// ## `secret_name`
    /// the secret key name to set flags for
    /// ## `flags`
    /// the `SettingSecretFlags` for the secret
    ///
    /// # Returns
    ///
    /// `true` on success (if the given secret name was a valid property of
    /// this setting, and if that property is secret), `false` if not
    fn set_secret_flags(
        &self,
        secret_name: &str,
        flags: SettingSecretFlags,
    ) -> Result<(), glib::Error>;

    /// Convert the setting (including secrets!) into a string. For debugging
    /// purposes ONLY, should NOT be used for serialization of the setting,
    /// or machine-parsed in any way. The output format is not guaranteed to
    /// be stable and may change at any time.
    ///
    /// # Returns
    ///
    /// an allocated string containing a textual representation of the
    /// setting's properties and values, which the caller should
    /// free with `g_free`
    fn to_string(&self) -> GString;

    /// Validates the setting. Each setting's properties have allowed values, and
    /// some are dependent on other values (hence the need for `connection`). The
    /// returned `glib::Error` contains information about which property of the setting
    /// failed validation, and in what way that property failed validation.
    /// ## `connection`
    /// the `Connection` that `self` came from, or
    ///  `None` if `self` is being verified in isolation.
    ///
    /// # Returns
    ///
    /// `true` if the setting is valid, `false` if it is not
    fn verify<P: IsA<Connection>>(&self, connection: Option<&P>) -> Result<(), glib::Error>;

    /// Verifies the secrets in the setting.
    /// The returned `glib::Error` contains information about which secret of the setting
    /// failed validation, and in what way that secret failed validation.
    /// The secret validation is done separately from main setting validation, because
    /// in some cases connection failure is not desired just for the secrets.
    ///
    /// Feature: `v1_2`
    ///
    /// ## `connection`
    /// the `Connection` that `self` came from, or
    ///  `None` if `self` is being verified in isolation.
    ///
    /// # Returns
    ///
    /// `true` if the setting secrets are valid, `false` if they are not
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn verify_secrets<P: IsA<Connection>>(&self, connection: Option<&P>)
        -> Result<(), glib::Error>;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Setting>> SettingExt for O {
    fn compare<P: IsA<Setting>>(&self, b: &P, flags: SettingCompareFlags) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_compare(
                self.as_ref().to_glib_none().0,
                b.as_ref().to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    //fn diff<P: IsA<Setting>>(&self, b: &P, flags: SettingCompareFlags, invert_results: bool, results: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 7 }) -> bool {
    //    unsafe { TODO: call nm_sys:nm_setting_diff() }
    //}

    fn duplicate(&self) -> Option<Setting> {
        unsafe { from_glib_full(nm_sys::nm_setting_duplicate(self.as_ref().to_glib_none().0)) }
    }

    //fn enumerate_values(&self, func: /*Unimplemented*/FnMut(&Setting, &str, /*Ignored*/glib::Value, /*Ignored*/glib::ParamFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call nm_sys:nm_setting_enumerate_values() }
    //}

    fn get_dbus_property_type(&self, property_name: &str) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_get_dbus_property_type(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_setting_get_name(self.as_ref().to_glib_none().0)) }
    }

    //#[cfg(any(feature = "v1_26", feature = "dox"))]
    //fn option_clear_by_name(&self, predicate: Option<&mut dyn (FnMut(&str) -> bool)>) {
    //    unsafe { TODO: call nm_sys:nm_setting_option_clear_by_name() }
    //}

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get(&self, opt_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_option_get(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get_all_names(&self) -> Vec<GString> {
        unsafe {
            let mut out_len = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                nm_sys::nm_setting_option_get_all_names(
                    self.as_ref().to_glib_none().0,
                    out_len.as_mut_ptr(),
                ),
                out_len.assume_init() as usize,
            );
            ret
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get_boolean(&self, opt_name: &str) -> Option<bool> {
        unsafe {
            let mut out_value = mem::MaybeUninit::uninit();
            let ret = from_glib(nm_sys::nm_setting_option_get_boolean(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                out_value.as_mut_ptr(),
            ));
            let out_value = out_value.assume_init();
            if ret {
                Some(from_glib(out_value))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_get_uint32(&self, opt_name: &str) -> Option<u32> {
        unsafe {
            let mut out_value = mem::MaybeUninit::uninit();
            let ret = from_glib(nm_sys::nm_setting_option_get_uint32(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                out_value.as_mut_ptr(),
            ));
            let out_value = out_value.assume_init();
            if ret {
                Some(out_value)
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_set(&self, opt_name: &str, variant: Option<&glib::Variant>) {
        unsafe {
            nm_sys::nm_setting_option_set(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                variant.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_set_boolean(&self, opt_name: &str, value: bool) {
        unsafe {
            nm_sys::nm_setting_option_set_boolean(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                value.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    fn option_set_uint32(&self, opt_name: &str, value: u32) {
        unsafe {
            nm_sys::nm_setting_option_set_uint32(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                value,
            );
        }
    }

    fn set_secret_flags(
        &self,
        secret_name: &str,
        flags: SettingSecretFlags,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_setting_set_secret_flags(
                self.as_ref().to_glib_none().0,
                secret_name.to_glib_none().0,
                flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn to_string(&self) -> GString {
        unsafe { from_glib_full(nm_sys::nm_setting_to_string(self.as_ref().to_glib_none().0)) }
    }

    fn verify<P: IsA<Connection>>(&self, connection: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_setting_verify(
                self.as_ref().to_glib_none().0,
                connection.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn verify_secrets<P: IsA<Connection>>(
        &self,
        connection: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_setting_verify_secrets(
                self.as_ref().to_glib_none().0,
                connection.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSetting,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Setting>,
        {
            let f: &F = &*(f as *const F);
            f(&Setting::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Setting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Setting")
    }
}
