// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//!
//! Planned verifier for Chapel programs: a subset gate and a proof-obligation front end over the Canonry theory. No code yet.
//!
//! The library half of the crate, so the behaviour is testable without spawning
//! the binary. Keep `main.rs` a thin shell around this.

/// One line describing what this program is, used by `--version` and the tests.
pub fn about() -> String {
    format!("chancel {}", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_names_the_package() {
        assert!(about().starts_with("chancel "));
    }
}
