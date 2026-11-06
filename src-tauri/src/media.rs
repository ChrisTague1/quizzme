use std::collections::HashSet;
use std::path::Path;

use uuid::Uuid;

/// SVG is excluded on purpose: it can carry script.
pub const ALLOWED_EXTS: [&str; 6] = ["png", "jpg", "gif", "webp", "bmp", "tiff"];

/// Writes image bytes to `media_dir` under a random name and returns the file name.
pub fn save(media_dir: &Path, bytes: &[u8], ext: &str) -> Result<String, String> {
    if bytes.is_empty() {
        return Err("Empty image".into());
    }
    if !ALLOWED_EXTS.contains(&ext) {
        return Err(format!("Unsupported image type: {ext}"));
    }
    let name = format!("{}.{}", Uuid::new_v4().simple(), ext);
    std::fs::write(media_dir.join(&name), bytes).map_err(|e| e.to_string())?;
    Ok(name)
}

/// Deletes unreferenced files. Startup only: an open editor may hold unsaved pasted images.
pub fn gc(media_dir: &Path, referenced: &HashSet<String>) {
    let entries = match std::fs::read_dir(media_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("media gc: cannot read {}: {e}", media_dir.display());
            return;
        }
    };
    for entry in entries.flatten() {
        let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
        if !is_file {
            continue;
        }
        let keep = entry.file_name().to_str().is_some_and(|n| referenced.contains(n));
        if !keep {
            if let Err(e) = std::fs::remove_file(entry.path()) {
                eprintln!("media gc: cannot remove {}: {e}", entry.path().display());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_input() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(save(dir.path(), b"x", "svg").unwrap_err(), "Unsupported image type: svg");
        assert_eq!(save(dir.path(), b"", "png").unwrap_err(), "Empty image");
    }

    #[test]
    fn saves_png_with_uuid_name() {
        let dir = tempfile::tempdir().unwrap();
        let name = save(dir.path(), b"data", "png").unwrap();
        let (stem, ext) = name.split_once('.').unwrap();
        assert_eq!(ext, "png");
        assert_eq!(stem.len(), 32);
        assert!(stem.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(std::fs::read(dir.path().join(&name)).unwrap(), b"data");
    }

    #[test]
    fn gc_removes_unreferenced_only() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.png"), b"a").unwrap();
        std::fs::write(dir.path().join("b.png"), b"b").unwrap();
        gc(dir.path(), &HashSet::from(["a.png".to_string()]));
        assert!(dir.path().join("a.png").exists());
        assert!(!dir.path().join("b.png").exists());
    }
}
