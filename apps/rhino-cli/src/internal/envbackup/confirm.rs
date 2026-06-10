//! Overwrite-confirmation helpers.

use std::io::{BufRead, Write};
use std::path::Path;

use super::types::FileEntry;

/// Returns the subset of `entries` whose destination paths already exist on
/// disk. For backup, `dest_root` is the backup directory; for restore, it is
/// the repo root.
pub fn find_existing(entries: &[FileEntry], dest_root: &str) -> Vec<String> {
    let mut existing = Vec::new();
    for e in entries {
        if e.skipped {
            continue;
        }
        let dst = Path::new(dest_root).join(&e.rel_path);
        if std::fs::metadata(&dst).is_ok() {
            existing.push(e.rel_path.clone());
        }
    }
    existing
}

/// Prints the list of conflicting files and prompts with `[y/N]`, reading the
/// answer from `reader` and writing the prompt to `writer`. Returns true only
/// for `y`/`yes` (case-insensitive).
pub fn default_confirm<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    existing: &[String],
) -> bool {
    let _ = writeln!(
        writer,
        "{} file(s) already exist. Overwrite? [y/N]",
        existing.len()
    );
    for p in existing {
        let _ = writeln!(writer, "  - {p}");
    }
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) > 0 {
        let answer = line.trim().to_lowercase();
        if answer == "y" || answer == "yes" {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::internal::envbackup::types::FileEntry;

    #[test]
    fn find_existing_reports_present_destinations() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join(".env"), b"x").unwrap();
        let entries = vec![
            FileEntry::new(".env".to_string(), "/src/.env".to_string(), 1),
            FileEntry::new("absent".to_string(), "/src/absent".to_string(), 1),
            FileEntry {
                rel_path: "skip".to_string(),
                abs_path: "/src/skip".to_string(),
                size: 0,
                skipped: true,
                reason: "symlink".to_string(),
                source: String::new(),
            },
        ];
        let existing = find_existing(&entries, &root.to_string_lossy());
        assert_eq!(existing, vec![".env".to_string()]);
    }

    #[test]
    fn confirm_accepts_yes_variants() {
        for ans in ["y\n", "Y\n", "yes\n", "YES\n"] {
            let mut r = std::io::Cursor::new(ans.as_bytes().to_vec());
            let mut w: Vec<u8> = Vec::new();
            assert!(default_confirm(&mut r, &mut w, &["a".to_string()]));
            let prompt = String::from_utf8(w).unwrap();
            assert!(prompt.contains("1 file(s) already exist. Overwrite? [y/N]"));
            assert!(prompt.contains("  - a"));
        }
    }

    #[test]
    fn confirm_rejects_other_answers() {
        for ans in ["n\n", "\n", "maybe\n", ""] {
            let mut r = std::io::Cursor::new(ans.as_bytes().to_vec());
            let mut w: Vec<u8> = Vec::new();
            assert!(!default_confirm(&mut r, &mut w, &["a".to_string()]));
        }
    }
}
