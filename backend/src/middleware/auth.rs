//! PIN-related helpers for the `todo` backend.
//!
//! The PIN verification, lockout, and rate-limit logic is delegated to
//! `shared_backend::auth::attempts` and `shared_backend::server::ip` so that
//! every studio2201 companion app uses the same primitives.
//!
//! This module retains only the **todo-specific** helpers:
//!
//! - Random 9-char task ID generation.
//! - File-format migrations for `data/todos.json`.

use shared_core::types::TodoLists;
use std::fs::File;
use std::io::Read;

/// Re-export the constant-time comparison from `shared-assets`'s
/// dependency. We deliberately do NOT roll our own.
pub use constant_time_eq::constant_time_eq as secure_compare;

/// Random 9-character alphanumeric task ID, used to give every task a
/// stable identity across edits and reordering.
pub fn generate_random_id() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut bytes = [0u8; 9];

    if let Ok(mut file) = File::open("/dev/urandom") {
        let _ = file.read_exact(&mut bytes);
    } else {
        // Fallback LCG using system time as seed.
        let mut seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        for byte in &mut bytes {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            *byte = (seed >> 32) as u8;
        }
    }

    bytes.iter_mut().for_each(|b| {
        *b = CHARSET[(*b as usize) % CHARSET.len()];
    });

    String::from_utf8_lossy(&bytes).into_owned()
}

/// Migrate any task items that lack an `id` field. Writes back in-place
/// atomically (temp + rename) if any IDs were assigned.
pub fn run_todo_migrations(data_file: &str) {
    if let Ok(content) = std::fs::read_to_string(data_file)
        && let Ok(mut lists) = serde_json::from_str::<TodoLists>(&content)
    {
        let mut updated = false;
        for items in lists.values_mut() {
            for item in items.iter_mut() {
                if item.id.is_empty() {
                    item.id = generate_random_id();
                    updated = true;
                }
            }
        }
        if updated && let Ok(serialized) = serde_json::to_string_pretty(&lists) {
            let temp_file = format!("{}.tmp", data_file);
            if std::fs::write(&temp_file, serialized).is_ok() {
                let _ = std::fs::rename(temp_file, data_file);
                println!("Migration: assigned unique IDs to tasks.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_id_is_9_alphanumeric_chars() {
        let id = generate_random_id();
        assert_eq!(id.len(), 9);
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn task_ids_are_different() {
        let a = generate_random_id();
        let b = generate_random_id();
        assert_ne!(a, b);
    }

    #[test]
    fn secure_compare_works() {
        assert!(secure_compare("1234".as_bytes(), "1234".as_bytes()));
        assert!(!secure_compare("1234".as_bytes(), "1235".as_bytes()));
        assert!(!secure_compare("1234".as_bytes(), "12345".as_bytes()));
        assert!(secure_compare(b"", b""));
    }
}
