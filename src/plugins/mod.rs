use std::fmt::Debug;

#[cfg(feature = "env")]
pub mod env;

#[cfg(feature = "gnome")]
pub mod gnome;

#[cfg(any(feature = "windows", windows))]
pub mod windows;

#[cfg(feature = "sysconfig")]
pub mod sysconfig;

#[cfg(feature = "osx")]
pub mod osx;

#[cfg(feature = "kde")]
pub mod kde;

#[cfg(feature = "xdp")]
pub mod xdp;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Plugin {}
