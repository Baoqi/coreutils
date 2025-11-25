// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! WASI-specific platform implementations for tail.
//! Process checking is not supported on WASI.

/// Process ID type for WASI (not actually used)
pub type Pid = i32;

/// Process checker for WASI - always reports process as dead since
/// we cannot check process status on WASI
pub struct ProcessChecker {
    _pid: Pid,
}

impl ProcessChecker {
    pub fn new(process_id: Pid) -> Self {
        Self { _pid: process_id }
    }

    /// On WASI, we cannot check process status, so always return true (dead)
    pub fn is_dead(&mut self) -> bool {
        true
    }
}

impl Drop for ProcessChecker {
    fn drop(&mut self) {}
}

/// On WASI, process checking is not supported
pub fn supports_pid_checks(_pid: Pid) -> bool {
    false
}

