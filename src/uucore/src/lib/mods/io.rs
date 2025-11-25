// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Encapsulates differences between OSs regarding the access to
//! file handles / descriptors.
//! This is useful when dealing with lower level stdin/stdout access.
//!
//! In detail:
//! On unix like OSs, file _descriptors_ are used in this context.
//! On windows OSs, file _handles_ are used.
//!
//! Even though they are distinct classes, they share common functionality.
//! Access to this common functionality is provided in `OwnedFileDescriptorOrHandle`.

#[cfg(all(not(windows), not(target_os = "wasi")))]
use std::os::fd::{AsFd, OwnedFd};
#[cfg(windows)]
use std::os::windows::io::{AsHandle, OwnedHandle};
use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
    process::Stdio,
};

#[cfg(windows)]
type NativeType = OwnedHandle;
#[cfg(all(not(windows), not(target_os = "wasi")))]
type NativeType = OwnedFd;
// On WASI, we use File directly as the native type since OwnedFd -> Stdio conversion is not stable
#[cfg(target_os = "wasi")]
type NativeType = File;

/// abstraction wrapper for native file handle / file descriptor
pub struct OwnedFileDescriptorOrHandle {
    fx: NativeType,
}

impl OwnedFileDescriptorOrHandle {
    /// create from underlying native type
    pub fn new(x: NativeType) -> Self {
        Self { fx: x }
    }

    /// create by opening a file
    pub fn open_file(options: &OpenOptions, path: &Path) -> io::Result<Self> {
        let f = options.open(path)?;
        Self::from(f)
    }

    /// conversion from borrowed native type
    ///
    /// e.g. `std::io::stdout()`, `std::fs::File`, ...
    #[cfg(windows)]
    pub fn from<T: AsHandle>(t: T) -> io::Result<Self> {
        Ok(Self {
            fx: t.as_handle().try_clone_to_owned()?,
        })
    }

    /// conversion from borrowed native type
    ///
    /// e.g. `std::io::stdout()`, `std::fs::File`, ...
    #[cfg(all(not(windows), not(target_os = "wasi")))]
    pub fn from<T: AsFd>(t: T) -> io::Result<Self> {
        Ok(Self {
            fx: t.as_fd().try_clone_to_owned()?,
        })
    }

    /// conversion from borrowed native type (WASI version)
    ///
    /// On WASI, we work with File directly
    #[cfg(target_os = "wasi")]
    pub fn from<T>(_t: T) -> io::Result<Self>
    where
        T: std::io::Read + std::io::Write,
    {
        // On WASI, we can't easily clone arbitrary file descriptors
        // This is a limitation - for now we return an error for non-File types
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "OwnedFileDescriptorOrHandle::from is not fully supported on WASI",
        ))
    }

    /// instantiates a corresponding `File`
    #[cfg(not(target_os = "wasi"))]
    pub fn into_file(self) -> File {
        File::from(self.fx)
    }

    /// instantiates a corresponding `File` (WASI version)
    #[cfg(target_os = "wasi")]
    pub fn into_file(self) -> File {
        self.fx
    }

    /// instantiates a corresponding `Stdio`
    #[cfg(not(target_os = "wasi"))]
    pub fn into_stdio(self) -> Stdio {
        Stdio::from(self.fx)
    }

    /// instantiates a corresponding `Stdio` (WASI version)
    #[cfg(target_os = "wasi")]
    pub fn into_stdio(self) -> Stdio {
        Stdio::from(self.fx)
    }

    /// clones self. useful when needing another
    /// owned reference to same file
    #[cfg(not(target_os = "wasi"))]
    pub fn try_clone(&self) -> io::Result<Self> {
        self.fx.try_clone().map(Self::new)
    }

    /// clones self (WASI version)
    #[cfg(target_os = "wasi")]
    pub fn try_clone(&self) -> io::Result<Self> {
        self.fx.try_clone().map(Self::new)
    }

    /// provides native type to be used with
    /// OS specific functions without abstraction
    pub fn as_raw(&self) -> &NativeType {
        &self.fx
    }
}

/// instantiates a corresponding `Stdio`
impl From<OwnedFileDescriptorOrHandle> for Stdio {
    fn from(value: OwnedFileDescriptorOrHandle) -> Self {
        value.into_stdio()
    }
}
