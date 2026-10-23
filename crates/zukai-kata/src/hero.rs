//! `kata::hero` — title + optional subtitle banner. The consistent header
//! every diagram opens with.

use zukai_model::Hero;

/// Args for [`hero`].
#[derive(Debug, Clone, Copy)]
pub struct HeroArgs<'a> {
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
}

/// Build the canonical hero banner from named args.
#[must_use]
pub fn hero(args: HeroArgs<'_>) -> Hero {
    Hero {
        title: args.title.into(),
        subtitle: args.subtitle.map(String::from),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_title_and_subtitle() {
        let h = hero(HeroArgs { title: "T", subtitle: Some("S") });
        assert_eq!(h.title, "T");
        assert_eq!(h.subtitle.as_deref(), Some("S"));
    }

    #[test]
    fn subtitle_is_optional() {
        let h = hero(HeroArgs { title: "T", subtitle: None });
        assert_eq!(h.title, "T");
        assert!(h.subtitle.is_none());
    }

    #[test]
    fn is_deterministic() {
        let a = hero(HeroArgs { title: "A", subtitle: Some("b") });
        let b = hero(HeroArgs { title: "A", subtitle: Some("b") });
        assert_eq!(a, b);
    }
}
