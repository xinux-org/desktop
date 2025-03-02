// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DesktopThumbnailSize;
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GnomeDesktopThumbnailFactory")]
    pub struct DesktopThumbnailFactory(Object<ffi::GnomeDesktopThumbnailFactory, ffi::GnomeDesktopThumbnailFactoryClass>);

    match fn {
        type_ => || ffi::gnome_desktop_thumbnail_factory_get_type(),
    }
}

impl DesktopThumbnailFactory {
    pub const NONE: Option<&'static DesktopThumbnailFactory> = None;

    #[doc(alias = "gnome_desktop_thumbnail_factory_new")]
    pub fn new(size: DesktopThumbnailSize) -> DesktopThumbnailFactory {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gnome_desktop_thumbnail_factory_new(size.into_glib())) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DesktopThumbnailFactory>> Sealed for T {}
}

pub trait DesktopThumbnailFactoryExt:
    IsA<DesktopThumbnailFactory> + sealed::Sealed + 'static
{
    #[doc(alias = "gnome_desktop_thumbnail_factory_can_thumbnail")]
    fn can_thumbnail(&self, uri: &str, mime_type: &str, mtime: libc::c_long) -> bool {
        unsafe {
            from_glib(ffi::gnome_desktop_thumbnail_factory_can_thumbnail(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                mime_type.to_glib_none().0,
                mtime,
            ))
        }
    }

    #[doc(alias = "gnome_desktop_thumbnail_factory_create_failed_thumbnail")]
    fn create_failed_thumbnail(
        &self,
        uri: &str,
        mtime: libc::c_long,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gnome_desktop_thumbnail_factory_create_failed_thumbnail(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                mtime,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gnome_desktop_thumbnail_factory_create_failed_thumbnail_async")]
    fn create_failed_thumbnail_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        uri: &str,
        original_mtime: libc::c_long,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn create_failed_thumbnail_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gnome_desktop_thumbnail_factory_create_failed_thumbnail_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = create_failed_thumbnail_async_trampoline::<P>;
        unsafe {
            ffi::gnome_desktop_thumbnail_factory_create_failed_thumbnail_async(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                original_mtime,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn create_failed_thumbnail_future(
        &self,
        uri: &str,
        original_mtime: libc::c_long,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let uri = String::from(uri);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.create_failed_thumbnail_async(
                &uri,
                original_mtime,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[cfg(feature = "v42")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v42")))]
    #[doc(alias = "gnome_desktop_thumbnail_factory_generate_thumbnail")]
    fn generate_thumbnail(
        &self,
        uri: &str,
        mime_type: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<gdk_pixbuf::Pixbuf, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gnome_desktop_thumbnail_factory_generate_thumbnail(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                mime_type.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gnome_desktop_thumbnail_factory_generate_thumbnail_async")]
    fn generate_thumbnail_async<P: FnOnce(Result<gdk_pixbuf::Pixbuf, glib::Error>) + 'static>(
        &self,
        uri: &str,
        mime_type: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn generate_thumbnail_async_trampoline<
            P: FnOnce(Result<gdk_pixbuf::Pixbuf, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gnome_desktop_thumbnail_factory_generate_thumbnail_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = generate_thumbnail_async_trampoline::<P>;
        unsafe {
            ffi::gnome_desktop_thumbnail_factory_generate_thumbnail_async(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                mime_type.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn generate_thumbnail_future(
        &self,
        uri: &str,
        mime_type: &str,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<gdk_pixbuf::Pixbuf, glib::Error>> + 'static>,
    > {
        let uri = String::from(uri);
        let mime_type = String::from(mime_type);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.generate_thumbnail_async(&uri, &mime_type, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gnome_desktop_thumbnail_factory_has_valid_failed_thumbnail")]
    fn has_valid_failed_thumbnail(&self, uri: &str, mtime: libc::c_long) -> bool {
        unsafe {
            from_glib(
                ffi::gnome_desktop_thumbnail_factory_has_valid_failed_thumbnail(
                    self.as_ref().to_glib_none().0,
                    uri.to_glib_none().0,
                    mtime,
                ),
            )
        }
    }

    #[doc(alias = "gnome_desktop_thumbnail_factory_lookup")]
    fn lookup(&self, uri: &str, mtime: libc::c_long) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gnome_desktop_thumbnail_factory_lookup(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                mtime,
            ))
        }
    }

    #[doc(alias = "gnome_desktop_thumbnail_factory_save_thumbnail")]
    fn save_thumbnail(
        &self,
        thumbnail: &gdk_pixbuf::Pixbuf,
        uri: &str,
        original_mtime: libc::c_long,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gnome_desktop_thumbnail_factory_save_thumbnail(
                self.as_ref().to_glib_none().0,
                thumbnail.to_glib_none().0,
                uri.to_glib_none().0,
                original_mtime,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gnome_desktop_thumbnail_factory_save_thumbnail_async")]
    fn save_thumbnail_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        thumbnail: &gdk_pixbuf::Pixbuf,
        uri: &str,
        original_mtime: libc::c_long,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn save_thumbnail_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gnome_desktop_thumbnail_factory_save_thumbnail_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = save_thumbnail_async_trampoline::<P>;
        unsafe {
            ffi::gnome_desktop_thumbnail_factory_save_thumbnail_async(
                self.as_ref().to_glib_none().0,
                thumbnail.to_glib_none().0,
                uri.to_glib_none().0,
                original_mtime,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn save_thumbnail_future(
        &self,
        thumbnail: &gdk_pixbuf::Pixbuf,
        uri: &str,
        original_mtime: libc::c_long,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let thumbnail = thumbnail.clone();
        let uri = String::from(uri);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save_thumbnail_async(
                &thumbnail,
                &uri,
                original_mtime,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }
}

impl<O: IsA<DesktopThumbnailFactory>> DesktopThumbnailFactoryExt for O {}

impl fmt::Display for DesktopThumbnailFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DesktopThumbnailFactory")
    }
}
