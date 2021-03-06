// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
use crate::VlanFlags;
use crate::VlanPriorityMap;
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
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct SettingVlan(Object<nm_sys::NMSettingVlan, nm_sys::NMSettingVlanClass, SettingVlanClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_vlan_get_type(),
    }
}

impl SettingVlan {
    /// Creates a new `SettingVlan` object with default values.
    ///
    /// # Returns
    ///
    /// the new empty `SettingVlan` object
    pub fn new() -> SettingVlan {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_vlan_new()).unsafe_cast() }
    }
}

impl Default for SettingVlan {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_VLAN: Option<&SettingVlan> = None;

/// Trait containing all `SettingVlan` methods.
///
/// # Implementors
///
/// [`SettingVlan`](struct.SettingVlan.html)
pub trait SettingVlanExt: 'static {
    /// Adds a priority mapping to the `SettingVlan:ingress_priority_map` or
    /// `SettingVlan:egress_priority_map` properties of the setting. If `from` is
    /// already in the given priority map, this function will overwrite the
    /// existing entry with the new `to`.
    ///
    /// If `map` is `VlanPriorityMap::IngressMap` then `from` is the incoming 802.1q VLAN
    /// Priority Code Point (PCP) value, and `to` is the Linux SKB priority value.
    ///
    /// If `map` is `VlanPriorityMap::EgressMap` then `from` is the Linux SKB priority value and
    /// `to` is the outgoing 802.1q VLAN Priority Code Point (PCP) value.
    /// ## `map`
    /// the type of priority map
    /// ## `from`
    /// the priority to map to `to`
    /// ## `to`
    /// the priority to map `from` to
    ///
    /// # Returns
    ///
    /// `true`.
    fn add_priority(&self, map: VlanPriorityMap, from: u32, to: u32) -> bool;

    /// Adds a priority map entry into either the `SettingVlan:ingress_priority_map`
    /// or the `SettingVlan:egress_priority_map` properties. The priority map maps
    /// the Linux SKB priorities to 802.1p priorities.
    /// ## `map`
    /// the type of priority map
    /// ## `str`
    /// the string which contains a priority map, like "3:7"
    ///
    /// # Returns
    ///
    /// `true` if the entry was successfully added to the list, or it
    /// overwrote the old value, `false` if `str` is not a valid mapping.
    fn add_priority_str(&self, map: VlanPriorityMap, str: &str) -> bool;

    /// Clear all the entries from `SettingVlan:ingress_priority_map` or
    /// `SettingVlan:egress_priority_map` properties.
    /// ## `map`
    /// the type of priority map
    fn clear_priorities(&self, map: VlanPriorityMap);

    ///
    /// # Returns
    ///
    /// the `SettingVlan:flags` property of the setting
    fn get_flags(&self) -> u32;

    ///
    /// # Returns
    ///
    /// the `SettingVlan:id` property of the setting
    fn get_id(&self) -> u32;

    /// Returns the number of entries in the
    /// `SettingVlan:ingress_priority_map` or `SettingVlan:egress_priority_map`
    /// properties of this setting.
    /// ## `map`
    /// the type of priority map
    ///
    /// # Returns
    ///
    /// return the number of ingress/egress priority entries.
    fn get_num_priorities(&self, map: VlanPriorityMap) -> i32;

    ///
    /// # Returns
    ///
    /// the `SettingVlan:parent` property of the setting
    fn get_parent(&self) -> Option<GString>;

    /// Retrieve one of the entries of the `SettingVlan:ingress_priority_map`
    /// or `SettingVlan:egress_priority_map` properties of this setting.
    /// ## `map`
    /// the type of priority map
    /// ## `idx`
    /// the zero-based index of the ingress/egress priority map entry
    /// ## `out_from`
    /// on return the value of the priority map's 'from' item
    /// ## `out_to`
    /// on return the value of priority map's 'to' item
    ///
    /// # Returns
    ///
    /// returns `true` if `idx` is in range. Otherwise `false`.
    fn get_priority(&self, map: VlanPriorityMap, idx: u32) -> Option<(u32, u32)>;

    /// Removes the priority map at index `idx` from the
    /// `SettingVlan:ingress_priority_map` or `SettingVlan:egress_priority_map`
    /// properties.
    /// ## `map`
    /// the type of priority map
    /// ## `idx`
    /// the zero-based index of the priority map to remove
    fn remove_priority(&self, map: VlanPriorityMap, idx: u32);

    /// Removes the priority map `form`:`to` from the `SettingVlan:ingress_priority_map`
    /// or `SettingVlan:egress_priority_map` (according to `map` argument)
    /// properties.
    /// ## `map`
    /// the type of priority map
    /// ## `from`
    /// the priority to map to `to`
    /// ## `to`
    /// the priority to map `from` to
    ///
    /// # Returns
    ///
    /// `true` if the priority mapping was found and removed; `false` if it was not.
    fn remove_priority_by_value(&self, map: VlanPriorityMap, from: u32, to: u32) -> bool;

    /// Removes the priority map `str` from the `SettingVlan:ingress_priority_map`
    /// or `SettingVlan:egress_priority_map` (according to `map` argument)
    /// properties.
    /// ## `map`
    /// the type of priority map
    /// ## `str`
    /// the string which contains a priority map, like "3:7"
    ///
    /// # Returns
    ///
    /// `true` if the priority mapping was found and removed; `false` if it was not.
    fn remove_priority_str_by_value(&self, map: VlanPriorityMap, str: &str) -> bool;

    /// For outgoing packets, a list of mappings from Linux SKB priorities to
    /// 802.1p priorities. The mapping is given in the format "from:to" where
    /// both "from" and "to" are unsigned integers, ie "7:3".
    fn get_property_egress_priority_map(&self) -> Vec<GString>;

    /// For outgoing packets, a list of mappings from Linux SKB priorities to
    /// 802.1p priorities. The mapping is given in the format "from:to" where
    /// both "from" and "to" are unsigned integers, ie "7:3".
    fn set_property_egress_priority_map(&self, egress_priority_map: &[&str]);

    /// One or more flags which control the behavior and features of the VLAN
    /// interface. Flags include `VlanFlags::ReorderHeaders` (reordering of
    /// output packet headers), `VlanFlags::Gvrp` (use of the GVRP protocol),
    /// and `VlanFlags::LooseBinding` (loose binding of the interface to its
    /// master device's operating state). `VlanFlags::Mvrp` (use of the MVRP
    /// protocol).
    ///
    /// The default value of this property is NM_VLAN_FLAG_REORDER_HEADERS,
    /// but it used to be 0. To preserve backward compatibility, the default-value
    /// in the D-Bus API continues to be 0 and a missing property on D-Bus
    /// is still considered as 0.
    fn set_property_flags(&self, flags: VlanFlags);

    /// The VLAN identifier that the interface created by this connection should
    /// be assigned. The valid range is from 0 to 4094, without the reserved id 4095.
    fn set_property_id(&self, id: u32);

    /// For incoming packets, a list of mappings from 802.1p priorities to Linux
    /// SKB priorities. The mapping is given in the format "from:to" where both
    /// "from" and "to" are unsigned integers, ie "7:3".
    fn get_property_ingress_priority_map(&self) -> Vec<GString>;

    /// For incoming packets, a list of mappings from 802.1p priorities to Linux
    /// SKB priorities. The mapping is given in the format "from:to" where both
    /// "from" and "to" are unsigned integers, ie "7:3".
    fn set_property_ingress_priority_map(&self, ingress_priority_map: &[&str]);

    /// If given, specifies the parent interface name or parent connection UUID
    /// from which this VLAN interface should be created. If this property is
    /// not specified, the connection must contain an `SettingWired` setting
    /// with a `SettingWired:mac-address` property.
    fn set_property_parent(&self, parent: Option<&str>);

    fn connect_property_egress_priority_map_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ingress_priority_map_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingVlan>> SettingVlanExt for O {
    fn add_priority(&self, map: VlanPriorityMap, from: u32, to: u32) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_vlan_add_priority(
                self.as_ref().to_glib_none().0,
                map.to_glib(),
                from,
                to,
            ))
        }
    }

    fn add_priority_str(&self, map: VlanPriorityMap, str: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_vlan_add_priority_str(
                self.as_ref().to_glib_none().0,
                map.to_glib(),
                str.to_glib_none().0,
            ))
        }
    }

    fn clear_priorities(&self, map: VlanPriorityMap) {
        unsafe {
            nm_sys::nm_setting_vlan_clear_priorities(self.as_ref().to_glib_none().0, map.to_glib());
        }
    }

    fn get_flags(&self) -> u32 {
        unsafe { nm_sys::nm_setting_vlan_get_flags(self.as_ref().to_glib_none().0) }
    }

    fn get_id(&self) -> u32 {
        unsafe { nm_sys::nm_setting_vlan_get_id(self.as_ref().to_glib_none().0) }
    }

    fn get_num_priorities(&self, map: VlanPriorityMap) -> i32 {
        unsafe {
            nm_sys::nm_setting_vlan_get_num_priorities(
                self.as_ref().to_glib_none().0,
                map.to_glib(),
            )
        }
    }

    fn get_parent(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_vlan_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_priority(&self, map: VlanPriorityMap, idx: u32) -> Option<(u32, u32)> {
        unsafe {
            let mut out_from = mem::MaybeUninit::uninit();
            let mut out_to = mem::MaybeUninit::uninit();
            let ret = from_glib(nm_sys::nm_setting_vlan_get_priority(
                self.as_ref().to_glib_none().0,
                map.to_glib(),
                idx,
                out_from.as_mut_ptr(),
                out_to.as_mut_ptr(),
            ));
            let out_from = out_from.assume_init();
            let out_to = out_to.assume_init();
            if ret {
                Some((out_from, out_to))
            } else {
                None
            }
        }
    }

    fn remove_priority(&self, map: VlanPriorityMap, idx: u32) {
        unsafe {
            nm_sys::nm_setting_vlan_remove_priority(
                self.as_ref().to_glib_none().0,
                map.to_glib(),
                idx,
            );
        }
    }

    fn remove_priority_by_value(&self, map: VlanPriorityMap, from: u32, to: u32) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_vlan_remove_priority_by_value(
                self.as_ref().to_glib_none().0,
                map.to_glib(),
                from,
                to,
            ))
        }
    }

    fn remove_priority_str_by_value(&self, map: VlanPriorityMap, str: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_vlan_remove_priority_str_by_value(
                self.as_ref().to_glib_none().0,
                map.to_glib(),
                str.to_glib_none().0,
            ))
        }
    }

    fn get_property_egress_priority_map(&self) -> Vec<GString> {
        unsafe {
            let mut value = Value::from_type(<Vec<GString> as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"egress-priority-map\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `egress-priority-map` getter")
                .unwrap()
        }
    }

    fn set_property_egress_priority_map(&self, egress_priority_map: &[&str]) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"egress-priority-map\0".as_ptr() as *const _,
                Value::from(egress_priority_map).to_glib_none().0,
            );
        }
    }

    fn set_property_flags(&self, flags: VlanFlags) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"flags\0".as_ptr() as *const _,
                Value::from(&flags).to_glib_none().0,
            );
        }
    }

    fn set_property_id(&self, id: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"id\0".as_ptr() as *const _,
                Value::from(&id).to_glib_none().0,
            );
        }
    }

    fn get_property_ingress_priority_map(&self) -> Vec<GString> {
        unsafe {
            let mut value = Value::from_type(<Vec<GString> as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ingress-priority-map\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ingress-priority-map` getter")
                .unwrap()
        }
    }

    fn set_property_ingress_priority_map(&self, ingress_priority_map: &[&str]) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ingress-priority-map\0".as_ptr() as *const _,
                Value::from(ingress_priority_map).to_glib_none().0,
            );
        }
    }

    fn set_property_parent(&self, parent: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"parent\0".as_ptr() as *const _,
                Value::from(parent).to_glib_none().0,
            );
        }
    }

    fn connect_property_egress_priority_map_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_egress_priority_map_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVlan::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::egress-priority-map\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_egress_priority_map_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVlan::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVlan::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_ingress_priority_map_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ingress_priority_map_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVlan::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ingress-priority-map\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ingress_priority_map_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVlan::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingVlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingVlan")
    }
}
