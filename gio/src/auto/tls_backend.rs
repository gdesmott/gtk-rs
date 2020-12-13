// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TlsDatabase;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct TlsBackend(Interface<ffi::GTlsBackend>);

    match fn {
        get_type => || ffi::g_tls_backend_get_type(),
    }
}

impl TlsBackend {
    #[doc(alias = "g_tls_backend_get_default")]
    pub fn get_default() -> Option<TlsBackend> {
        unsafe { from_glib_none(ffi::g_tls_backend_get_default()) }
    }
}

pub const NONE_TLS_BACKEND: Option<&TlsBackend> = None;

pub trait TlsBackendExt: 'static {
    #[doc(alias = "g_tls_backend_get_certificate_type")]
    fn get_certificate_type(&self) -> glib::types::Type;

    #[doc(alias = "g_tls_backend_get_client_connection_type")]
    fn get_client_connection_type(&self) -> glib::types::Type;

    #[doc(alias = "g_tls_backend_get_default_database")]
    fn get_default_database(&self) -> Option<TlsDatabase>;

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
    #[doc(alias = "g_tls_backend_get_dtls_client_connection_type")]
    fn get_dtls_client_connection_type(&self) -> glib::types::Type;

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
    #[doc(alias = "g_tls_backend_get_dtls_server_connection_type")]
    fn get_dtls_server_connection_type(&self) -> glib::types::Type;

    #[doc(alias = "g_tls_backend_get_file_database_type")]
    fn get_file_database_type(&self) -> glib::types::Type;

    #[doc(alias = "g_tls_backend_get_server_connection_type")]
    fn get_server_connection_type(&self) -> glib::types::Type;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_tls_backend_set_default_database")]
    fn set_default_database<P: IsA<TlsDatabase>>(&self, database: Option<&P>);

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
    #[doc(alias = "g_tls_backend_supports_dtls")]
    fn supports_dtls(&self) -> bool;

    #[doc(alias = "g_tls_backend_supports_tls")]
    fn supports_tls(&self) -> bool;
}

impl<O: IsA<TlsBackend>> TlsBackendExt for O {
    fn get_certificate_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_certificate_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_client_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_client_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_default_database(&self) -> Option<TlsDatabase> {
        unsafe {
            from_glib_full(ffi::g_tls_backend_get_default_database(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
    fn get_dtls_client_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_dtls_client_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
    fn get_dtls_server_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_dtls_server_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_file_database_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_file_database_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_server_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_server_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn set_default_database<P: IsA<TlsDatabase>>(&self, database: Option<&P>) {
        unsafe {
            ffi::g_tls_backend_set_default_database(
                self.as_ref().to_glib_none().0,
                database.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
    fn supports_dtls(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_backend_supports_dtls(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn supports_tls(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_backend_supports_tls(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for TlsBackend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsBackend")
    }
}