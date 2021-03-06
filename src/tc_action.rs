// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::translate::*;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::GString;
use nm_sys;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct TCAction(Shared<nm_sys::NMTCAction>);

    match fn {
        ref => |ptr| nm_sys::nm_tc_action_ref(ptr),
        unref => |ptr| nm_sys::nm_tc_action_unref(ptr),
        get_type => || nm_sys::nm_tc_action_get_type(),
    }
}

impl TCAction {
    /// Creates a new `TCAction` object.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `kind`
    /// name of the queueing discipline
    ///
    /// # Returns
    ///
    /// the new `TCAction` object, or `None` on error
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new(kind: &str) -> Result<TCAction, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_tc_action_new(kind.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Creates a copy of `self`
    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// a copy of `self`
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn dup(&self) -> Option<TCAction> {
        unsafe { from_glib_full(nm_sys::nm_tc_action_dup(self.to_glib_none().0)) }
    }

    /// Determines if two `TCAction` objects contain the same kind, family,
    /// handle, parent and info.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `other`
    /// the `TCAction` to compare `self` to.
    ///
    /// # Returns
    ///
    /// `true` if the objects contain the same values, `false` if they do not.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn equal(&self, other: &TCAction) -> bool {
        unsafe {
            from_glib(nm_sys::nm_tc_action_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    /// Gets the value of the attribute with name `name` on `self`
    /// ## `name`
    /// the name of an action attribute
    ///
    /// # Returns
    ///
    /// the value of the attribute with name `name` on
    ///  `self`, or `None` if `self` has no such attribute.
    pub fn get_attribute(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(nm_sys::nm_tc_action_get_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_12`
    ///
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_kind(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_tc_action_get_kind(self.to_glib_none().0)) }
    }

    /// Sets or clears the named attribute on `self` to the given value.
    /// ## `name`
    /// the name of an action attribute
    /// ## `value`
    /// the value
    pub fn set_attribute(&self, name: &str, value: Option<&glib::Variant>) {
        unsafe {
            nm_sys::nm_tc_action_set_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

impl PartialEq for TCAction {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TCAction {}
