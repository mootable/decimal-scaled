// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `UrlLoader` — fetches golden files over HTTPS and caches them locally, so the
//! published crate ships the harness ONLY (the ~130 MB golden set is far over the
//! crates.io package limit) while still working out of the box. Each function's file
//! is downloaded ONCE into a local cache, then parsed by a [`FileLoader`] over that
//! cache — so all parsing / header / missing-file behaviour is shared, never
//! duplicated. Behind the `net` feature so the core crate stays dependency-free.
//!
//! The default loader pins to a minor-resolving TAG derived from this crate's version
//! (`v<major>.<minor>`, e.g. `v0.5`): a moving alias the release process points at the
//! latest patch of the line. So a published crate always fetches its own minor's
//! newest (non-regressing, by the versioning policy) corpus — "0.5 → 0.5.max" —
//! without a code change, while never silently crossing a minor/major.

use std::borrow::Cow;
use std::path::PathBuf;

use crate::function::Function;
use crate::subject::Limits;

use super::file::FileLoader;
use super::loader::{CaseLoader, GoldenCase};

/// The repository the default loader fetches the golden set from.
const REPO_RAW: &str = "https://raw.githubusercontent.com/mootable/decimal-scaled";
/// The golden directory within the repo.
const GOLDEN_SUBDIR: &str = "decimal-scaled-golden/golden";
/// Env var overriding the local cache root (default: a `decimal-scaled-golden`
/// subdir of the system temp dir).
const CACHE_ENV: &str = "DECIMAL_SCALED_GOLDEN_CACHE";

/// The git ref [`UrlLoader::default`] pins to: the minor-resolving tag
/// `v<major>.<minor>` of THIS crate's version (e.g. `v0.5`). The release process
/// maintains that moving tag to point at the latest patch of the line, so the default
/// corpus is the minor's max patch ("0.5 → 0.5.max"). Override via [`UrlLoader::from_ref`].
pub const DEFAULT_REF: &str =
    concat!("v", env!("CARGO_PKG_VERSION_MAJOR"), ".", env!("CARGO_PKG_VERSION_MINOR"));

/// A [`CaseLoader`] that fetches golden files over HTTPS and caches them locally.
/// Lazy + per-function: only the files for the functions actually loaded are
/// downloaded, once each; thereafter the cached copy is reused (offline).
pub struct UrlLoader {
    base_url: String,
    cache: PathBuf,
}

impl UrlLoader {
    /// A loader over an explicit `base_url` (each request is `base_url` + `<fn>.au`,
    /// so it should end with `/`), caching into `cache_dir`.
    pub fn new(base_url: impl Into<String>, cache_dir: impl Into<PathBuf>) -> UrlLoader {
        let cache = cache_dir.into();
        let _ = std::fs::create_dir_all(&cache);
        UrlLoader { base_url: base_url.into(), cache }
    }

    /// A loader pinned to a git `git_ref` of the decimal-scaled repo, caching under a
    /// per-ref subdir so different refs never collide.
    pub fn from_ref(git_ref: &str) -> UrlLoader {
        let base_url = format!("{REPO_RAW}/{git_ref}/{GOLDEN_SUBDIR}/");
        UrlLoader::new(base_url, cache_root().join(git_ref))
    }

    /// The local cache directory this loader downloads into.
    pub fn cache_dir(&self) -> &std::path::Path {
        &self.cache
    }

    /// Best-effort download of one function's file into the cache (a no-op if already
    /// cached). A network / HTTP failure is reported to stderr and leaves the file
    /// absent, so [`CaseLoader::load`] then yields no cases — the same contract as a
    /// `FileLoader` over a missing file.
    fn ensure_cached(&self, func: Function) {
        let path = self.cache.join(format!("{}.au", func.name()));
        if path.exists() {
            return;
        }
        let url = format!("{}{}.au", self.base_url, func.name());
        match ureq::get(&url).call() {
            // ureq 3: the body is read via `body_mut()`. Raise the read limit well
            // above the largest golden file (~8 MB) so a big `.au` is never truncated.
            Ok(mut resp) => match resp.body_mut().with_config().limit(64 * 1024 * 1024).read_to_string() {
                Ok(body) => {
                    if let Err(e) = std::fs::write(&path, body) {
                        eprintln!("UrlLoader: caching {}: {e}", path.display());
                    }
                }
                Err(e) => eprintln!("UrlLoader: reading {url}: {e}"),
            },
            Err(e) => eprintln!("UrlLoader: fetching {url}: {e}"),
        }
    }
}

impl Default for UrlLoader {
    /// The repo's golden set at [`DEFAULT_REF`] (the `v<major>.<minor>` tag).
    fn default() -> UrlLoader {
        UrlLoader::from_ref(DEFAULT_REF)
    }
}

impl CaseLoader for UrlLoader {
    fn load(&self, func: Function) -> Cow<'_, [GoldenCase]> {
        self.ensure_cached(func);
        // Delegate parse + missing-file handling to a FileLoader over the cache, so the
        // golden format is parsed in exactly one place.
        Cow::Owned(FileLoader::new(&self.cache).load(func).into_owned())
    }

    fn oracle_limits(&self) -> Limits {
        // The `#gen_precision`/`#guard` header from whatever is cached (or the
        // documented default if nothing is yet) — the same source as FileLoader.
        FileLoader::new(&self.cache).oracle_limits()
    }
}

/// The cache root: `$DECIMAL_SCALED_GOLDEN_CACHE`, else `<temp>/decimal-scaled-golden`.
fn cache_root() -> PathBuf {
    std::env::var_os(CACHE_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("decimal-scaled-golden"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_ref_is_the_minor_tag() {
        // Derived from this crate's version, so a 0.5.x build pins to `v0.5`.
        assert!(DEFAULT_REF.starts_with('v'));
        assert_eq!(DEFAULT_REF.matches('.').count(), 1, "major.minor only");
    }

    #[test]
    fn from_ref_builds_a_pinned_repo_url_and_per_ref_cache() {
        let l = UrlLoader::from_ref("v9.9");
        assert!(l.base_url.ends_with("/v9.9/decimal-scaled-golden/golden/"));
        assert!(l.cache_dir().ends_with("v9.9"));
    }
}
