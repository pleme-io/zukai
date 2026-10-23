//! The CLI's render path, end to end: every target writes a non-empty
//! artifact, and two renders of the same diagram are byte-equal.

use std::process::Command;

fn render(target: &str, out: &std::path::Path) {
    let status = Command::new(env!("CARGO_BIN_EXE_zukai"))
        .args(["render", "--target", target, "--out"])
        .arg(out)
        .status()
        .expect("spawn zukai");
    assert!(status.success(), "zukai render --target {target} failed");
}

#[test]
fn every_target_renders_deterministically() {
    let dir = std::env::temp_dir().join(format!("zukai-cli-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for target in ["svg", "png", "mermaid"] {
        let a = dir.join(format!("a.{target}"));
        let b = dir.join(format!("b.{target}"));
        render(target, &a);
        render(target, &b);
        let (a, b) = (std::fs::read(&a).unwrap(), std::fs::read(&b).unwrap());
        assert!(!a.is_empty(), "{target}: empty artifact");
        assert_eq!(a, b, "{target}: two renders differ");
    }
    let svg = std::fs::read_to_string(dir.join("a.svg")).unwrap();
    assert!(svg.contains("<svg"), "svg target did not emit svg");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn targets_lists_all_three() {
    let out = Command::new(env!("CARGO_BIN_EXE_zukai")).arg("targets").output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8(out.stdout).unwrap();
    for t in ["Svg", "Png", "Mermaid"] {
        assert!(s.contains(t), "missing {t} in {s:?}");
    }
}
