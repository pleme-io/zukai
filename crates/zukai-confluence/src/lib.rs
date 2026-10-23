//! zukai-confluence — Confluence storage-XML primitives for **documentation
//! expression**. The documentation half of zukai (the diagram half lives in
//! `zukai-render`, embedded via `zukai_render::confluence::embed`).
//!
//! Every primitive:
//! - Returns `String` (owned, ready to splice into a page body).
//! - XML-escapes user input where unsafe.
//! - Is deterministic (same input → same output).
//!
//! ## v0 surface
//!
//! - **Panels** (`info` / `warning` / `note` / `tip` / `success`) — `panel`.
//! - **Structure** (`expand`, `code`, `toc`, `tabs`, `tasklist`, `status`) — `structure`.
//! - **Layout** (`two_column`, `three_column`) — `layout`.
//! - **Inline** (`paragraph`, `hr`, `link`, `page_link`, `mention`) — `inline`.
//! - **Table** (header + rows builder) — `table`.
//!
//! ## Why this exists
//!
//! Before v0.1 zukai could only embed diagrams (`<ac:image>`); the rest of a
//! Confluence page was hand-authored XML. zukai-confluence types every
//! non-diagram structural primitive so a page body can be **composed**
//! entirely from Rust functions — no string-templating, no escape mistakes,
//! no copy-paste from Atlassian docs.
//!
//! ## Composing with diagrams
//!
//! ```text
//! let body = format!("{intro}{panel}{diagram_embed}{closing}",
//!     intro = inline::paragraph("Overview"),
//!     panel = panel::info("Read this first."),
//!     diagram_embed = zukai_render::confluence::embed("arch.png", 840),
//!     closing = layout::two_column(panel::tip("See …"), inline::link("docs", "https://…")),
//! );
//! ```

#![forbid(unsafe_code)]

pub mod inline;
pub mod layout;
pub mod host;
pub mod page;
pub mod panel;
pub mod structure;
pub mod table;
pub mod xml;

pub use page::Page;

pub use inline::{hr, link, mention, page_link, paragraph};
pub use layout::{three_column, two_column};
pub use panel::{info, note, success, tip, warning};
pub use structure::{code, expand, status, tabs, tasklist, toc};
pub use table::table;
