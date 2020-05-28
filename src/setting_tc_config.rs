// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::object::Cast;
use glib::translate::*;
use nm_sys;
use std::fmt;
use Setting;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use TCQdisc;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use TCTfilter;

glib_wrapper! {
    pub struct SettingTCConfig(Object<nm_sys::NMSettingTCConfig, nm_sys::NMSettingTCConfigClass, SettingTCConfigClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_tc_config_get_type(),
    }
}

impl SettingTCConfig {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new() -> SettingTCConfig {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_tc_config_new()).unsafe_cast() }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn add_qdisc(&self, qdisc: &TCQdisc) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_tc_config_add_qdisc(
                self.to_glib_none().0,
                qdisc.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn add_tfilter(&self, tfilter: &TCTfilter) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_tc_config_add_tfilter(
                self.to_glib_none().0,
                tfilter.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn clear_qdiscs(&self) {
        unsafe {
            nm_sys::nm_setting_tc_config_clear_qdiscs(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn clear_tfilters(&self) {
        unsafe {
            nm_sys::nm_setting_tc_config_clear_tfilters(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_num_qdiscs(&self) -> u32 {
        unsafe { nm_sys::nm_setting_tc_config_get_num_qdiscs(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_num_tfilters(&self) -> u32 {
        unsafe { nm_sys::nm_setting_tc_config_get_num_tfilters(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_qdisc(&self, idx: u32) -> Option<TCQdisc> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_tc_config_get_qdisc(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_tfilter(&self, idx: u32) -> Option<TCTfilter> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_tc_config_get_tfilter(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn remove_qdisc(&self, idx: u32) {
        unsafe {
            nm_sys::nm_setting_tc_config_remove_qdisc(self.to_glib_none().0, idx);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn remove_qdisc_by_value(&self, qdisc: &TCQdisc) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_tc_config_remove_qdisc_by_value(
                self.to_glib_none().0,
                qdisc.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn remove_tfilter(&self, idx: u32) {
        unsafe {
            nm_sys::nm_setting_tc_config_remove_tfilter(self.to_glib_none().0, idx);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn remove_tfilter_by_value(&self, tfilter: &TCTfilter) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_tc_config_remove_tfilter_by_value(
                self.to_glib_none().0,
                tfilter.to_glib_none().0,
            ))
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl Default for SettingTCConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingTCConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingTCConfig")
    }
}