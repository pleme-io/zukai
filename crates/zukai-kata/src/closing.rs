//! `kata::closing` — bottom summary/closing band. The mirror of `hero` at
//! the end of a diagram. Wide single-line panel with optional sublabel.

use zukai_model::{Band, Rect, Role};

/// Args for [`closing`].
#[derive(Debug, Clone, Copy)]
pub struct ClosingArgs<'a> {
    pub label: &'a str,
    pub sublabel: Option<&'a str>,
    pub role: Role,
}

/// Build a closing band suitable as a diagram footer.
#[must_use]
pub fn closing(args: ClosingArgs<'_>) -> Band {
    Band {
        label: args.label.into(),
        sublabel: args.sublabel.map(String::from),
        role: args.role,
        at: Rect { x: 40, y: 510, w: 880, h: 40 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_label_role_and_position() {
        let c = closing(ClosingArgs { label: "Summary", sublabel: None, role: Role::Accent });
        assert_eq!(c.label, "Summary");
        assert_eq!(c.role, Role::Accent);
        assert_eq!(c.at.y, 510);
        assert_eq!(c.at.w, 880);
    }

    #[test]
    fn sublabel_is_optional() {
        let c = closing(ClosingArgs { label: "x", sublabel: Some("s"), role: Role::Neutral });
        assert_eq!(c.sublabel.as_deref(), Some("s"));
    }

    #[test]
    fn is_deterministic() {
        let a = closing(ClosingArgs { label: "x", sublabel: None, role: Role::Neutral });
        let b = closing(ClosingArgs { label: "x", sublabel: None, role: Role::Neutral });
        assert_eq!(a, b);
    }
}
