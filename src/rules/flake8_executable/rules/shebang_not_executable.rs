#[cfg(not(target_family = "wasm"))]
use std::os::unix::prelude::MetadataExt;
use std::path::Path;

use ruff_macros::derive_message_formats;
#[cfg(not(target_family = "wasm"))]
use rustpython_ast::Location;

#[cfg(not(target_family = "wasm"))]
use crate::ast::types::Range;
use crate::define_violation;
use crate::registry::Diagnostic;
use crate::rules::flake8_executable::helpers::ShebangDirective;
use crate::violation::Violation;

define_violation!(
    pub struct ShebangNotExecutable;
);
impl Violation for ShebangNotExecutable {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Shebang is present but file is not executable")
    }
}

/// EXE001
#[cfg(not(target_family = "wasm"))]
pub fn shebang_not_executable(
    filepath: &Path,
    lineno: usize,
    shebang: &ShebangDirective,
) -> Option<Diagnostic> {
    if let ShebangDirective::Match(_, start, end, _) = shebang {
        if let Ok(metadata) = filepath.metadata() {
            // Check if file is executable by anyone
            if metadata.mode() & 0o111 == 0 {
                let diagnostic = Diagnostic::new(
                    ShebangNotExecutable,
                    Range::new(
                        Location::new(lineno + 1, *start),
                        Location::new(lineno + 1, *end),
                    ),
                );
                Some(diagnostic)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(target_family = "wasm")]
pub fn shebang_not_executable(
    _filepath: &Path,
    _lineno: usize,
    _shebang: &ShebangDirective,
) -> Option<Diagnostic> {
    None
}
