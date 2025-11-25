// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#[cfg(not(target_os = "wasi"))]
mod files;
#[cfg(not(target_os = "wasi"))]
mod watch;

#[cfg(not(target_os = "wasi"))]
pub use watch::{Observer, follow};

#[cfg(target_os = "wasi")]
mod wasi;
#[cfg(target_os = "wasi")]
pub use wasi::{Observer, follow};
