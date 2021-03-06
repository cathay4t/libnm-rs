// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Connection;
use crate::VpnEditor;
use crate::VpnEditorPluginCapability;
#[cfg(any(feature = "v1_4", feature = "dox"))]
use crate::VpnPluginInfo;
use glib;
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
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct VpnEditorPlugin(Interface<nm_sys::NMVpnEditorPlugin>);

    match fn {
        get_type => || nm_sys::nm_vpn_editor_plugin_get_type(),
    }
}

impl VpnEditorPlugin {
    /// Load the shared library `plugin_name` and create a new
    /// `VpnEditorPlugin` instance via the `NMVpnEditorPluginFactory`
    /// function.
    ///
    /// This is similar to `VpnEditorPlugin::load_from_file`, but
    /// it does no validation of the plugin name, instead passes it directly
    /// to `dlopen`. If you have the full path to a plugin file,
    /// `VpnEditorPlugin::load_from_file` is preferred.
    ///
    /// Feature: `v1_4`
    ///
    /// ## `plugin_name`
    /// The name of the shared library to load.
    ///  This path will be directly passed to `dlopen` without
    ///  further checks.
    /// ## `check_service`
    /// if not-null, check that the loaded plugin advertises
    ///  the given service.
    ///
    /// # Returns
    ///
    /// a new plugin instance or `None` on error.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn load(plugin_name: &str, check_service: &str) -> Result<VpnEditorPlugin, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_vpn_editor_plugin_load(
                plugin_name.to_glib_none().0,
                check_service.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[cfg(any(feature = "v1_2", feature = "dox"))]
    //pub fn load_from_file<P: FnMut(&str, Option<&glib::Error>) -> bool>(plugin_name: &str, check_service: &str, check_owner: i32, check_file: P) -> Result<VpnEditorPlugin, glib::Error> {
    //    unsafe { TODO: call nm_sys:nm_vpn_editor_plugin_load_from_file() }
    //}
}

pub const NONE_VPN_EDITOR_PLUGIN: Option<&VpnEditorPlugin> = None;

/// Trait containing all `VpnEditorPlugin` methods.
///
/// # Implementors
///
/// [`VpnEditorPlugin`](struct.VpnEditorPlugin.html)
pub trait VpnEditorPluginExt: 'static {
    fn export<P: IsA<Connection>>(&self, path: &str, connection: &P) -> Result<(), glib::Error>;

    fn get_capabilities(&self) -> VpnEditorPluginCapability;

    /// ## `connection`
    /// the `Connection` to be edited
    ///
    /// # Returns
    ///
    /// a new `VpnEditor` or `None` on error
    fn get_editor<P: IsA<Connection>>(&self, connection: &P) -> Result<VpnEditor, glib::Error>;

    ///
    /// Feature: `v1_4`
    ///
    ///
    /// # Returns
    ///
    /// if set, return the `VpnPluginInfo` instance.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_plugin_info(&self) -> Option<VpnPluginInfo>;

    fn get_suggested_filename<P: IsA<Connection>>(&self, connection: &P) -> Option<GString>;

    //#[cfg(any(feature = "v1_4", feature = "dox"))]
    //fn get_vt(&self, vt: /*Ignored*/VpnEditorPluginVT, vt_size: usize) -> usize;

    /// ## `path`
    /// full path to the file to attempt to read into a new `Connection`
    ///
    /// # Returns
    ///
    /// a new `Connection` imported from `path`, or `None`
    /// on error or if the file at `path` was not recognized by this plugin
    fn import(&self, path: &str) -> Result<Connection, glib::Error>;

    /// Set or clear the plugin-info instance.
    /// This takes a weak reference on `plugin_info`, to avoid circular
    /// reference as the plugin-info might also reference the editor-plugin.
    ///
    /// Feature: `v1_4`
    ///
    /// ## `plugin_info`
    /// a `VpnPluginInfo` instance or `None`
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_plugin_info(&self, plugin_info: Option<&VpnPluginInfo>);

    /// Longer description of the VPN plugin.
    fn get_property_description(&self) -> Option<GString>;

    /// Short display name of the VPN plugin.
    fn get_property_name(&self) -> Option<GString>;

    /// D-Bus service name of the plugin's VPN service.
    fn get_property_service(&self) -> Option<GString>;

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VpnEditorPlugin>> VpnEditorPluginExt for O {
    fn export<P: IsA<Connection>>(&self, path: &str, connection: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_vpn_editor_plugin_export(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_capabilities(&self) -> VpnEditorPluginCapability {
        unsafe {
            from_glib(nm_sys::nm_vpn_editor_plugin_get_capabilities(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_editor<P: IsA<Connection>>(&self, connection: &P) -> Result<VpnEditor, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_vpn_editor_plugin_get_editor(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_plugin_info(&self) -> Option<VpnPluginInfo> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_editor_plugin_get_plugin_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_suggested_filename<P: IsA<Connection>>(&self, connection: &P) -> Option<GString> {
        unsafe {
            from_glib_full(nm_sys::nm_vpn_editor_plugin_get_suggested_filename(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_4", feature = "dox"))]
    //fn get_vt(&self, vt: /*Ignored*/VpnEditorPluginVT, vt_size: usize) -> usize {
    //    unsafe { TODO: call nm_sys:nm_vpn_editor_plugin_get_vt() }
    //}

    fn import(&self, path: &str) -> Result<Connection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_vpn_editor_plugin_import(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_plugin_info(&self, plugin_info: Option<&VpnPluginInfo>) {
        unsafe {
            nm_sys::nm_vpn_editor_plugin_set_plugin_info(
                self.as_ref().to_glib_none().0,
                plugin_info.to_glib_none().0,
            );
        }
    }

    fn get_property_description(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"description\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `description` getter")
        }
    }

    fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `name` getter")
        }
    }

    fn get_property_service(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"service\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `service` getter")
        }
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMVpnEditorPlugin,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnEditorPlugin>,
        {
            let f: &F = &*(f as *const F);
            f(&VpnEditorPlugin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMVpnEditorPlugin,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnEditorPlugin>,
        {
            let f: &F = &*(f as *const F);
            f(&VpnEditorPlugin::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_service_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMVpnEditorPlugin,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnEditorPlugin>,
        {
            let f: &F = &*(f as *const F);
            f(&VpnEditorPlugin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::service\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_service_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for VpnEditorPlugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VpnEditorPlugin")
    }
}
