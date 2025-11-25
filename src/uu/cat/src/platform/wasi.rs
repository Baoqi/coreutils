// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// WASI platform-specific implementation for cat.
/// On WASI, we have limited file system capabilities, so we provide
/// a simplified implementation that doesn't check for unsafe overwrites
/// (WASI runs in a sandbox anyway).

/// Check if reading from input and writing to output would cause an unsafe overwrite.
/// On WASI, we always return false since:
/// 1. WASI runs in a sandboxed environment
/// 2. We don't have access to low-level file descriptor operations like fstat
pub fn is_unsafe_overwrite<I, O>(_input: &I, _output: &O) -> bool {
    false
}

