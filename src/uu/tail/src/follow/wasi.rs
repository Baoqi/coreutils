// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! WASI-specific implementation of tail follow functionality.
//! On WASI, the follow (-f) feature is not supported due to lack of
//! file system notification APIs.

use crate::args::Settings;
use std::io::BufRead;
use uucore::error::{UResult, USimpleError};
use uucore::translate;

/// Observer for WASI - follow mode is not supported
pub struct Observer {
    // Empty struct for WASI
}

impl From<&Settings> for Observer {
    fn from(_settings: &Settings) -> Self {
        Self {}
    }
}

impl Observer {
    pub fn add_path(
        &mut self,
        _path: &std::path::Path,
        _display_name: &str,
        _reader: Option<Box<dyn BufRead>>,
        _update_last: bool,
    ) -> UResult<()> {
        Ok(())
    }

    pub fn add_bad_path(
        &mut self,
        _path: &std::path::Path,
        _display_name: &str,
        _update_last: bool,
    ) -> UResult<()> {
        Ok(())
    }

    pub fn add_stdin(
        &mut self,
        _display_name: &str,
        _reader: Option<Box<dyn BufRead>>,
        _update_last: bool,
    ) -> UResult<()> {
        Ok(())
    }

    pub fn follow_name_retry(&self) -> bool {
        false
    }

    pub fn start(&mut self, settings: &Settings) -> UResult<()> {
        // On WASI, if follow mode is requested, return an error
        if settings.follow.is_some() {
            return Err(USimpleError::new(
                1,
                translate!("tail-error-follow-not-supported-wasi"),
            ));
        }
        Ok(())
    }
}

/// Follow function for WASI - not supported
pub fn follow(_observer: &mut Observer, _settings: &Settings) -> UResult<()> {
    Err(USimpleError::new(
        1,
        translate!("tail-error-follow-not-supported-wasi"),
    ))
}

