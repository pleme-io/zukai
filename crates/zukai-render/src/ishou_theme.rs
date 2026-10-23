//! `ishou`-backed Theme. The first real pleme-io wiring — `ishou_tokens::TokenSet`
//! (Nord palette, pleme generic) adapted to zukai's `Theme` trait via
//! pre-resolution of each `SemanticRoles` key to its `ColorPalette` hex.
//!
//! Generic + public. Brand-specific theming lives in a separate private
//! `bordado` overlay (in a private consumer crate); it never lives here. The
//! `from_tokens` constructor is the seam where such overlays plug in by
//! populating a `TokenSet` with their own values.

use crate::{Role, Theme};
use ishou_tokens::TokenSet;

/// Theme backed by an `ishou_tokens::TokenSet`. Pre-resolves every role to a
/// hex string at construction so the `Theme` trait can hand out `&str`.
#[derive(Debug, Clone)]
pub struct IshouTheme {
    bg: String,
    panel: String,
    fg: String,
    muted: String,
    border: String,
    font: String,
    role_neutral: String,
    role_master: String,
    role_read: String,
    role_accent: String,
    role_success: String,
    role_warning: String,
    role_danger: String,
    role_info: String,
}

impl IshouTheme {
    /// The canonical pleme generic theme (Nord palette, dark binding).
    #[must_use]
    pub fn pleme() -> Self {
        Self::from_tokens(&TokenSet::pleme())
    }

    /// Build from any `TokenSet`. This is the overlay seam — a private
    /// `bordado` populates its own `TokenSet` and constructs an `IshouTheme`
    /// from it.
    #[must_use]
    pub fn from_tokens(tokens: &TokenSet) -> Self {
        let palette = &tokens.color;
        let roles = &tokens.roles;
        let resolve = |key: &str| -> String {
            palette
                .get(key)
                .map(|r| r.hex())
                .unwrap_or_else(|| "#000000".into())
        };
        Self {
            bg: resolve(roles.background),
            panel: resolve(roles.surface),
            fg: resolve(roles.text),
            muted: resolve(roles.text_muted),
            border: resolve(roles.border),
            font: tokens.typography.families.sans.to_string(),
            role_neutral: resolve(roles.text_muted),
            role_master: resolve(roles.primary),
            role_read: resolve(roles.info),
            role_accent: resolve(roles.accent),
            role_success: resolve(roles.success),
            role_warning: resolve(roles.warning),
            role_danger: resolve(roles.error),
            role_info: resolve(roles.info),
        }
    }
}

impl Default for IshouTheme {
    fn default() -> Self {
        Self::pleme()
    }
}

impl Theme for IshouTheme {
    fn bg(&self) -> &str { &self.bg }
    fn panel(&self) -> &str { &self.panel }
    fn fg(&self) -> &str { &self.fg }
    fn muted(&self) -> &str { &self.muted }
    fn border(&self) -> &str { &self.border }
    fn font(&self) -> &str { &self.font }
    fn role(&self, role: Role) -> &str {
        match role {
            Role::Neutral => &self.role_neutral,
            Role::Master => &self.role_master,
            Role::Read => &self.role_read,
            Role::Accent => &self.role_accent,
            Role::Success => &self.role_success,
            Role::Warning => &self.role_warning,
            Role::Danger => &self.role_danger,
            Role::Info => &self.role_info,
        }
    }
}
