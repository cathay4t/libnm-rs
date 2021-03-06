// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_10", feature = "dox"))]
use crate::ActivationStateFlags;
use crate::ActiveConnectionState;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use crate::ActiveConnectionStateReason;
use crate::Device;
use crate::DhcpConfig;
use crate::IPConfig;
use crate::Object;
use crate::RemoteConnection;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use libc;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ActiveConnection(Object<nm_sys::NMActiveConnection, nm_sys::NMActiveConnectionClass, ActiveConnectionClass>) @extends Object;

    match fn {
        get_type => || nm_sys::nm_active_connection_get_type(),
    }
}

pub const NONE_ACTIVE_CONNECTION: Option<&ActiveConnection> = None;

/// Trait containing all `ActiveConnection` methods.
///
/// # Implementors
///
/// [`ActiveConnection`](struct.ActiveConnection.html), [`VpnConnection`](struct.VpnConnection.html)
pub trait ActiveConnectionExt: 'static {
    /// Gets the `RemoteConnection` associated with `self`.
    ///
    /// # Returns
    ///
    /// the `RemoteConnection` which this
    /// `ActiveConnection` is an active instance of.
    fn get_connection(&self) -> Option<RemoteConnection>;

    /// Gets the `Connection`'s type.
    ///
    /// # Returns
    ///
    /// the type of the `Connection` that backs the `ActiveConnection`.
    /// This is the internal string used by the connection, and must not be modified.
    fn get_connection_type(&self) -> Option<GString>;

    /// Whether the active connection is the default IPv4 one (that is, is used for
    /// the default IPv4 route and DNS information).
    ///
    /// # Returns
    ///
    /// `true` if the active connection is the default IPv4 connection
    fn get_default(&self) -> bool;

    /// Whether the active connection is the default IPv6 one (that is, is used for
    /// the default IPv6 route and DNS information).
    ///
    /// # Returns
    ///
    /// `true` if the active connection is the default IPv6 connection
    fn get_default6(&self) -> bool;

    /// Gets the `NMDevices` used for the active connections.
    ///
    /// # Returns
    ///
    /// the `glib::PtrArray` containing `NMDevices`.
    /// This is the internal copy used by the connection, and must not be modified.
    fn get_devices(&self) -> Vec<Device>;

    /// Gets the current IPv4 `DhcpConfig` (if any) associated with the
    /// `ActiveConnection`.
    ///
    /// # Returns
    ///
    /// the IPv4 `DhcpConfig`, or `None` if the connection
    ///  does not use DHCP, or is not in the `ActiveConnectionState::Activated`
    ///  state.
    fn get_dhcp4_config(&self) -> Option<DhcpConfig>;

    /// Gets the current IPv6 `DhcpConfig` (if any) associated with the
    /// `ActiveConnection`.
    ///
    /// # Returns
    ///
    /// the IPv6 `DhcpConfig`, or `None` if the connection
    ///  does not use DHCPv6, or is not in the `ActiveConnectionState::Activated`
    ///  state.
    fn get_dhcp6_config(&self) -> Option<DhcpConfig>;

    /// Gets the `Connection`'s ID.
    ///
    /// # Returns
    ///
    /// the ID of the `Connection` that backs the `ActiveConnection`.
    /// This is the internal string used by the connection, and must not be modified.
    fn get_id(&self) -> Option<GString>;

    /// Gets the current IPv4 `IPConfig` associated with the `ActiveConnection`.
    ///
    /// # Returns
    ///
    /// the IPv4 `IPConfig`, or `None` if the connection is
    ///  not in the `ActiveConnectionState::Activated` state.
    fn get_ip4_config(&self) -> Option<IPConfig>;

    /// Gets the current IPv6 `IPConfig` associated with the `ActiveConnection`.
    ///
    /// # Returns
    ///
    /// the IPv6 `IPConfig`, or `None` if the connection is
    ///  not in the `ActiveConnectionState::Activated` state.
    fn get_ip6_config(&self) -> Option<IPConfig>;

    /// Gets the master `Device` of the connection.
    ///
    /// # Returns
    ///
    /// the master `Device` of the `ActiveConnection`.
    fn get_master(&self) -> Option<Device>;

    /// Gets the path of the "specific object" used at activation.
    ///
    /// Currently there is no single method that will allow you to automatically turn
    /// this into an appropriate `Object`; you need to know what kind of object it
    /// is based on other information. (Eg, if `self` corresponds to a Wi-Fi
    /// connection, then the specific object will be an `AccessPoint`, and you can
    /// resolve it with `DeviceWifi::get_access_point_by_path`.)
    ///
    /// # Returns
    ///
    /// the specific object's D-Bus path. This is the internal string used
    /// by the connection, and must not be modified.
    fn get_specific_object_path(&self) -> Option<GString>;

    /// Gets the active connection's state.
    ///
    /// # Returns
    ///
    /// the state
    fn get_state(&self) -> ActiveConnectionState;

    /// Gets the active connection's state flags.
    ///
    /// Feature: `v1_10`
    ///
    ///
    /// # Returns
    ///
    /// the state flags
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_state_flags(&self) -> ActivationStateFlags;

    /// Gets the reason for active connection's state.
    ///
    /// Feature: `v1_8`
    ///
    ///
    /// # Returns
    ///
    /// the reason
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn get_state_reason(&self) -> ActiveConnectionStateReason;

    /// Gets the `Connection`'s UUID.
    ///
    /// # Returns
    ///
    /// the UUID of the `Connection` that backs the `ActiveConnection`.
    /// This is the internal string used by the connection, and must not be modified.
    fn get_uuid(&self) -> Option<GString>;

    /// Whether the active connection is a VPN connection.
    ///
    /// # Returns
    ///
    /// `true` if the active connection is a VPN connection
    fn get_vpn(&self) -> bool;

    /// The active connection's type
    fn get_property_type(&self) -> Option<GString>;

    /// ## `state`
    /// the new state number (`ActiveConnectionState`)
    /// ## `reason`
    /// the state change reason (`ActiveConnectionStateReason`)
    fn connect_state_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_connection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default6_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_dhcp4_config_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_dhcp6_config_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip4_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip6_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_specific_object_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_state_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vpn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ActiveConnection>> ActiveConnectionExt for O {
    fn get_connection(&self) -> Option<RemoteConnection> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_connection_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_default(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_default(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_default6(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_default6(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_active_connection_get_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_dhcp4_config(&self) -> Option<DhcpConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_dhcp4_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_dhcp6_config(&self) -> Option<DhcpConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_dhcp6_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_ip4_config(&self) -> Option<IPConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_ip4_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_ip6_config(&self) -> Option<IPConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_ip6_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_master(&self) -> Option<Device> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_master(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_specific_object_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_specific_object_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_state(&self) -> ActiveConnectionState {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_state_flags(&self) -> ActivationStateFlags {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_state_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn get_state_reason(&self) -> ActiveConnectionStateReason {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_state_reason(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uuid(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_uuid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_vpn(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_vpn(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
        }
    }

    fn connect_state_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<P, F: Fn(&P, u32, u32) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            state: libc::c_uint,
            reason: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ActiveConnection::from_glib_borrow(this).unsafe_cast_ref(),
                state,
                reason,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    state_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_connection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_connection_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::connection\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_connection_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_default6_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default6_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default6\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default6_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_devices_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::devices\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_devices_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_dhcp4_config_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dhcp4_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dhcp4-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dhcp4_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_dhcp6_config_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dhcp6_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dhcp6-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dhcp6_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ip4_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip4_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ip4-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ip4_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ip6_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip6_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ip6-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ip6_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_master_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::master\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_master_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_specific_object_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_specific_object_path_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::specific-object-path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_specific_object_path_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_state_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uuid_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::uuid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_uuid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_vpn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vpn_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vpn\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vpn_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ActiveConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActiveConnection")
    }
}
