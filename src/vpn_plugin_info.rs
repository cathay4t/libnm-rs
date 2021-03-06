// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_2", feature = "dox"))]
use crate::VpnEditorPlugin;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::object::IsA;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib_sys;
use nm_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    pub struct VpnPluginInfo(Object<nm_sys::NMVpnPluginInfo, nm_sys::NMVpnPluginInfoClass, VpnPluginInfoClass>);

    match fn {
        get_type => || nm_sys::nm_vpn_plugin_info_get_type(),
    }
}

impl VpnPluginInfo {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn from_file(filename: &str) -> Result<VpnPluginInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                nm_sys::nm_vpn_plugin_info_new_from_file(filename.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// This has the same effect as doing a full `nm_vpn_plugin_info_list_load`
    /// followed by a search for the first matching VPN plugin info that has the
    /// given `name` and/or `service`.
    ///
    /// Feature: `v1_4`
    ///
    /// ## `name`
    /// the name to search for. Either `name` or `service`
    ///  must be present.
    /// ## `service`
    /// the service to search for. Either `name` or
    ///  `service` must be present.
    ///
    /// # Returns
    ///
    /// a newly created instance of plugin info
    ///  or `None` if no matching value was found.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn new_search_file(name: Option<&str>, service: Option<&str>) -> VpnPluginInfo {
        unsafe {
            from_glib_full(nm_sys::nm_vpn_plugin_info_new_search_file(
                name.to_glib_none().0,
                service.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn with_data(
        filename: &str,
        keyfile: &glib::KeyFile,
    ) -> Result<VpnPluginInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_vpn_plugin_info_new_with_data(
                filename.to_glib_none().0,
                keyfile.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    ///
    /// Feature: `v1_4`
    ///
    ///
    /// # Returns
    ///
    ///
    ///  the aliases from the name-file.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn get_aliases(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_vpn_plugin_info_get_aliases(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_4`
    ///
    ///
    /// # Returns
    ///
    /// the absolute path to the auth-dialog helper or `None`.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn get_auth_dialog(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_auth_dialog(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Returns
    ///
    /// the cached `VpnEditorPlugin` instance.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_editor_plugin(&self) -> Option<VpnEditorPlugin> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_editor_plugin(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Returns
    ///
    /// the filename. Can be `None`.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_filename(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_filename(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Returns
    ///
    /// the name. Cannot be `None`.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_vpn_plugin_info_get_name(self.to_glib_none().0)) }
    }

    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Returns
    ///
    /// the plugin. Can be `None`.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_plugin(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_vpn_plugin_info_get_plugin(self.to_glib_none().0)) }
    }

    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Returns
    ///
    /// the program. Can be `None`.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_program(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_program(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_4`
    ///
    ///
    /// # Returns
    ///
    /// the service. Cannot be `None`.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn get_service(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_service(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Returns
    ///
    /// loads the plugin and returns the newly created
    ///  instance. The plugin is owned by `self` and can be later retrieved again
    ///  via `VpnPluginInfo::get_editor_plugin`. You can load the
    ///  plugin only once, unless you reset the state via
    ///  `VpnPluginInfo::set_editor_plugin`.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn load_editor_plugin(&self) -> Result<VpnEditorPlugin, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                nm_sys::nm_vpn_plugin_info_load_editor_plugin(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    ///
    /// Feature: `v1_2`
    ///
    /// ## `group`
    /// group name
    /// ## `key`
    /// name of the property
    ///
    /// # Returns
    ///
    /// `VpnPluginInfo` is internally a `glib::KeyFile`. Returns the matching
    /// property.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn lookup_property(&self, group: &str, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_lookup_property(
                self.to_glib_none().0,
                group.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    /// Set the internal plugin instance. If `None`, only clear the previous instance.
    ///
    /// Feature: `v1_2`
    ///
    /// ## `plugin`
    /// plugin instance
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn set_editor_plugin<P: IsA<VpnEditorPlugin>>(&self, plugin: Option<&P>) {
        unsafe {
            nm_sys::nm_vpn_plugin_info_set_editor_plugin(
                self.to_glib_none().0,
                plugin.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    ///
    /// Feature: `v1_4`
    ///
    ///
    /// # Returns
    ///
    /// `true` if the supports hints for secret requests, otherwise `false`
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn supports_hints(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_vpn_plugin_info_supports_hints(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_2`
    ///
    ///
    /// # Returns
    ///
    /// `true` if the service supports multiple instances with different bus names, otherwise `false`
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn supports_multiple(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_vpn_plugin_info_supports_multiple(
                self.to_glib_none().0,
            ))
        }
    }

    /// Regular name files have a certain pattern. That basically means
    /// they have the file extension "name". Check if `filename`
    /// is valid according to that pattern.
    ///
    /// Feature: `v1_2`
    ///
    /// ## `filename`
    /// the filename to check
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn validate_filename(filename: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_vpn_plugin_info_validate_filename(
                filename.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_name_notify<F: Fn(&VpnPluginInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&VpnPluginInfo) + 'static>(
            this: *mut nm_sys::NMVpnPluginInfo,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for VpnPluginInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VpnPluginInfo")
    }
}
