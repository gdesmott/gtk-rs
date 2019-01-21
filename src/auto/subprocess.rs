// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_40", feature = "dox"))]
use Cancellable;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use Error;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use InputStream;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use OutputStream;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use SubprocessFlags;
use ffi;
#[cfg(feature = "futures")]
#[cfg(any(feature = "v2_40", feature = "dox"))]
use futures_core;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use gobject_ffi;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use std;
#[cfg(feature = "futures")]
#[cfg(any(feature = "v2_40", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    pub struct Subprocess(Object<ffi::GSubprocess, SubprocessClass>);

    match fn {
        get_type => || ffi::g_subprocess_get_type(),
    }
}

impl Subprocess {
    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //pub fn new<'a, P: Into<Option<&'a Error>>>(flags: SubprocessFlags, error: P, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Subprocess {
    //    unsafe { TODO: call ffi::g_subprocess_new() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn newv(argv: &[&std::ffi::OsStr], flags: SubprocessFlags) -> Result<Subprocess, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_subprocess_newv(argv.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn communicate<'a, 'b, P: Into<Option<&'a glib::Bytes>>, Q: IsA<Cancellable> + 'b, R: Into<Option<&'b Q>>>(&self, stdin_buf: P, cancellable: R) -> Result<(Option<glib::Bytes>, Option<glib::Bytes>), Error> {
        let stdin_buf = stdin_buf.into();
        let cancellable = cancellable.into();
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate(self.to_glib_none().0, stdin_buf.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut stdout_buf, &mut stderr_buf, &mut error);
            if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn communicate_async<'a, 'b, P: Into<Option<&'a glib::Bytes>>, Q: IsA<Cancellable> + 'b, R: Into<Option<&'b Q>>, S: FnOnce(Result<(glib::Bytes, glib::Bytes), Error>) + Send + 'static>(&self, stdin_buf: P, cancellable: R, callback: S) {
        let stdin_buf = stdin_buf.into();
        let cancellable = cancellable.into();
        let user_data: Box<Box<S>> = Box::new(Box::new(callback));
        unsafe extern "C" fn communicate_async_trampoline<S: FnOnce(Result<(glib::Bytes, glib::Bytes), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_finish(_source_object as *mut _, res, &mut stdout_buf, &mut stderr_buf, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<S>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = communicate_async_trampoline::<S>;
        unsafe {
            ffi::g_subprocess_communicate_async(self.to_glib_none().0, stdin_buf.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn communicate_async_future<'a, P: Into<Option<&'a glib::Bytes>>>(&self, stdin_buf: P) -> Box_<futures_core::Future<Item = (Self, (glib::Bytes, glib::Bytes)), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let stdin_buf = stdin_buf.into();
        let stdin_buf = stdin_buf.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.communicate_async(
                 stdin_buf.as_ref().map(::std::borrow::Borrow::borrow),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn communicate_utf8<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Cancellable> + 'b, R: Into<Option<&'b Q>>>(&self, stdin_buf: P, cancellable: R) -> Result<(Option<GString>, Option<GString>), Error> {
        let stdin_buf = stdin_buf.into();
        let cancellable = cancellable.into();
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_utf8(self.to_glib_none().0, stdin_buf.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut stdout_buf, &mut stderr_buf, &mut error);
            if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn force_exit(&self) {
        unsafe {
            ffi::g_subprocess_force_exit(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_exit_status(&self) -> i32 {
        unsafe {
            ffi::g_subprocess_get_exit_status(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_identifier(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_identifier(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_if_exited(&self) -> bool {
        unsafe {
            from_glib(ffi::g_subprocess_get_if_exited(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_if_signaled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_subprocess_get_if_signaled(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_status(&self) -> i32 {
        unsafe {
            ffi::g_subprocess_get_status(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_stderr_pipe(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_stderr_pipe(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_stdin_pipe(&self) -> Option<OutputStream> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_stdin_pipe(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_stdout_pipe(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_stdout_pipe(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_successful(&self) -> bool {
        unsafe {
            from_glib(ffi::g_subprocess_get_successful(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_term_sig(&self) -> i32 {
        unsafe {
            ffi::g_subprocess_get_term_sig(self.to_glib_none().0)
        }
    }

    #[cfg(any(not(windows), feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn send_signal(&self, signal_num: i32) {
        unsafe {
            ffi::g_subprocess_send_signal(self.to_glib_none().0, signal_num);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn wait<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, cancellable: Q) -> Result<(), Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn wait_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn wait_async_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_async_trampoline::<R>;
        unsafe {
            ffi::g_subprocess_wait_async(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn wait_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.wait_async(
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn wait_check<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, cancellable: Q) -> Result<(), Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_check(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn wait_check_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn wait_check_async_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_check_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_check_async_trampoline::<R>;
        unsafe {
            ffi::g_subprocess_wait_check_async(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn wait_check_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.wait_check_async(
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }
}

impl fmt::Display for Subprocess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Subprocess")
    }
}
