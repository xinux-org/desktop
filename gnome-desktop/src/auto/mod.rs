// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

mod desktop_thumbnail_factory;
pub use self::desktop_thumbnail_factory::DesktopThumbnailFactory;

mod idle_monitor;
pub use self::idle_monitor::IdleMonitor;

mod pnp_ids;
pub use self::pnp_ids::PnpIds;

mod wall_clock;
pub use self::wall_clock::WallClock;

mod xkb_info;
pub use self::xkb_info::XkbInfo;

mod enums;
pub use self::enums::DesktopThumbnailSize;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::desktop_thumbnail_factory::DesktopThumbnailFactoryExt;
    pub use super::idle_monitor::IdleMonitorExt;
    pub use super::pnp_ids::PnpIdsExt;
    pub use super::wall_clock::WallClockExt;
    pub use super::xkb_info::XkbInfoExt;
}
#[doc(hidden)]
pub mod builders {
    pub use super::wall_clock::WallClockBuilder;
}
