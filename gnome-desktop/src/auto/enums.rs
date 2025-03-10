// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GnomeDesktopThumbnailSize")]
pub enum DesktopThumbnailSize {
    #[doc(alias = "GNOME_DESKTOP_THUMBNAIL_SIZE_NORMAL")]
    Normal,
    #[doc(alias = "GNOME_DESKTOP_THUMBNAIL_SIZE_LARGE")]
    Large,
    #[doc(alias = "GNOME_DESKTOP_THUMBNAIL_SIZE_XLARGE")]
    Xlarge,
    #[doc(alias = "GNOME_DESKTOP_THUMBNAIL_SIZE_XXLARGE")]
    Xxlarge,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DesktopThumbnailSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DesktopThumbnailSize::{}",
            match *self {
                Self::Normal => "Normal",
                Self::Large => "Large",
                Self::Xlarge => "Xlarge",
                Self::Xxlarge => "Xxlarge",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for DesktopThumbnailSize {
    type GlibType = ffi::GnomeDesktopThumbnailSize;

    #[inline]
    fn into_glib(self) -> ffi::GnomeDesktopThumbnailSize {
        match self {
            Self::Normal => ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_NORMAL,
            Self::Large => ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_LARGE,
            Self::Xlarge => ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_XLARGE,
            Self::Xxlarge => ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_XXLARGE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GnomeDesktopThumbnailSize> for DesktopThumbnailSize {
    #[inline]
    unsafe fn from_glib(value: ffi::GnomeDesktopThumbnailSize) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_NORMAL => Self::Normal,
            ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_LARGE => Self::Large,
            ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_XLARGE => Self::Xlarge,
            ffi::GNOME_DESKTOP_THUMBNAIL_SIZE_XXLARGE => Self::Xxlarge,
            value => Self::__Unknown(value),
        }
    }
}
