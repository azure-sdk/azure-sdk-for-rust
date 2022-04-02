#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(all(feature = "package-preview-2022-04", not(feature = "no-default-version")))]
pub use package_preview_2022_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-version")))]
pub use package_2022_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "no-default-version")))]
pub use package_preview_2022_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-11")]
pub mod package_preview_2021_11;
#[cfg(all(feature = "package-preview-2021-11", not(feature = "no-default-version")))]
pub use package_preview_2021_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-09")]
pub mod package_2021_09;
#[cfg(all(feature = "package-2021-09", not(feature = "no-default-version")))]
pub use package_2021_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
