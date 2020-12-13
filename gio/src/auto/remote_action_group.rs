// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ActionGroup;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct RemoteActionGroup(Interface<ffi::GRemoteActionGroup>) @requires ActionGroup;

    match fn {
        get_type => || ffi::g_remote_action_group_get_type(),
    }
}

pub const NONE_REMOTE_ACTION_GROUP: Option<&RemoteActionGroup> = None;

pub trait RemoteActionGroupExt: 'static {
    #[doc(alias = "g_remote_action_group_activate_action_full")]
    fn activate_action_full(
        &self,
        action_name: &str,
        parameter: Option<&glib::Variant>,
        platform_data: &glib::Variant,
    );

    #[doc(alias = "g_remote_action_group_change_action_state_full")]
    fn change_action_state_full(
        &self,
        action_name: &str,
        value: &glib::Variant,
        platform_data: &glib::Variant,
    );
}

impl<O: IsA<RemoteActionGroup>> RemoteActionGroupExt for O {
    fn activate_action_full(
        &self,
        action_name: &str,
        parameter: Option<&glib::Variant>,
        platform_data: &glib::Variant,
    ) {
        unsafe {
            ffi::g_remote_action_group_activate_action_full(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                parameter.to_glib_none().0,
                platform_data.to_glib_none().0,
            );
        }
    }

    fn change_action_state_full(
        &self,
        action_name: &str,
        value: &glib::Variant,
        platform_data: &glib::Variant,
    ) {
        unsafe {
            ffi::g_remote_action_group_change_action_state_full(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                value.to_glib_none().0,
                platform_data.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for RemoteActionGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RemoteActionGroup")
    }
}