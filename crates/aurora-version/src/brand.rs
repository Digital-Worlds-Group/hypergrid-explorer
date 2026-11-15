//! HyperGrid Explorer's identity: the one place that names this fork of
//! Aurora Viewer. Every crate reads it from here, so an upstream merge only
//! touches the lines that use these constants.
//!
//! HyperGrid Explorer is a modified version of Aurora Viewer (GPL-3.0-or-later,
//! see NOTICE.md). Aurora's TRADEMARKS.md asks a modified version to use its own
//! name and icon, keep Aurora's notices and say that it was modified: this module
//! holds the name; the notices stay in NOTICE.md and the About window.

/// The product name shown to users: window title, About, installer, exe resources.
pub const NAME: &str = "HyperGrid Explorer";

/// The name with no space, for identifiers (user agent, cache folder, IDs).
pub const ID: &str = "HyperGridExplorer";

/// The viewer this fork is based on, named in About and NOTICE.md.
pub const UPSTREAM: &str = "Aurora Viewer";

/// The Aurora Viewer version HyperGrid Explorer was forked from (its
/// `git describe` from Aurora's tags): shown in About, Preferences and the
/// README. **Update these three together after each merge from upstream.**
pub const UPSTREAM_VERSION: &str = "0.9.10-beta.20261115.1+17.g03a8b9d15";
/// That Aurora Viewer commit, in full.
pub const UPSTREAM_COMMIT: &str = "03a8b9d15a7824ddd2bb22afeb26af28141eb36d";
/// That commit's date.
pub const UPSTREAM_DATE: &str = "2026-11-15";
/// Aurora Viewer's source repository.
pub const UPSTREAM_SOURCE: &str = "https://github.com/Aurora-Viewer/Aurora-Viewer";

/// Who maintains this fork.
pub const OWNER: &str = "Terry Ford";

/// The fork's contact address, shown in About.
pub const OWNER_CONTACT: &str = "terryf43080@outlook.com";

/// When the fork started modifying Aurora Viewer (GPLv3 section 5(a)).
pub const MODIFIED_SINCE: &str = "2026-11-15";

/// The fork's website.
pub const WEBSITE: &str = "https://hypergridexplorer.com";

/// `directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)`:
/// on Windows `%APPDATA%\HyperGridExplorer\HyperGridExplorer` and
/// `%LOCALAPPDATA%\HyperGridExplorer\HyperGridExplorer`, apart from Aurora's
/// `Aurora\AuroraViewer`, so both viewers can live on one PC.
pub const QUALIFIER: &str = "com";
pub const ORGANIZATION: &str = "HyperGridExplorer";
pub const APPLICATION: &str = "HyperGridExplorer";

/// The Windows AppUserModelID (notifications, taskbar grouping).
pub const APP_USER_MODEL_ID: &str = "HyperGridExplorer.Viewer";

/// Prefix of the credential-manager entries (passwords, MFA, cache key), so
/// they never collide with an installed Aurora Viewer's.
pub const CREDENTIAL_SERVICE: &str = "HyperGrid Explorer";

/// The GitHub repository whose releases the updater follows and where crash
/// and bug reports go, as `owner/name`. `None` until the fork has its public
/// repository: updates are then announced on the website only, and reports
/// are saved locally instead of being posted to Aurora's tracker.
pub const GITHUB_REPO: Option<&str> = None;

/// Rewrites the product name in an interface text written for Aurora Viewer.
///
/// "Aurora Viewer" and "Aurora" naming the program become [`NAME`]; the names
/// that are not the program stay, because the grid, files or scripts depend
/// on them: the "#Aurora" inventory folders and the "Aurora Library", the
/// "Aurora bridge" (an in-world script), the "Aurora pose" file format, the
/// "Aurora nameplates" style, and the places of the offline demo.
pub fn rebrand(text: &str) -> std::borrow::Cow<'_, str> {
    if !text.contains("Aurora") {
        return std::borrow::Cow::Borrowed(text);
    }
    const KEEP_AFTER: &[&str] = &[
        " bridge",
        " Bridge",
        " Library",
        " pose",
        " nameplate",
        " Plaza",
        " Lagoon",
        " Demo",
        " demo",
        "/",
    ];
    let mut out = String::with_capacity(text.len() + 32);
    let mut rest = text;
    while let Some(at) = rest.find("Aurora") {
        let (before, from) = rest.split_at(at);
        out.push_str(before);
        let after = &from["Aurora".len()..];
        let prev = out.chars().next_back();
        let next = after.chars().next();
        let word = !prev.is_some_and(|c| c.is_alphanumeric() || c == '#' || c == '_' || c == '-')
            && !next.is_some_and(|c| c.is_alphanumeric() || c == '_');
        if !word || KEEP_AFTER.iter().any(|k| after.starts_with(k)) {
            out.push_str("Aurora");
            rest = after;
        } else if let Some(tail) = after.strip_prefix(" Viewer") {
            out.push_str(NAME);
            rest = tail;
        } else {
            out.push_str(NAME);
            rest = after;
        }
    }
    out.push_str(rest);
    std::borrow::Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::rebrand;

    #[test]
    fn the_program_is_renamed() {
        assert_eq!(rebrand("Aurora Viewer 0.9 (Dev)"), "HyperGrid Explorer 0.9 (Dev)");
        assert_eq!(
            rebrand("Aurora's web and media support"),
            "HyperGrid Explorer's web and media support"
        );
        assert_eq!(
            rebrand("Applies to every Aurora window."),
            "Applies to every HyperGrid Explorer window."
        );
        assert_eq!(rebrand("Aurora Viewer's own"), "HyperGrid Explorer's own");
    }

    #[test]
    fn names_the_grid_or_files_depend_on_stay() {
        for keep in [
            "Use the Aurora bridge",
            "Place resources in #Aurora/Aurora Library",
            "Aurora pose",
            "Colour the name on Aurora nameplates",
            "Sunset over Aurora Plaza",
            "Aurora Lagoon",
            "Welcome to the Aurora demo estate",
            "AuroraViewerCache",
            "aurora-plaza",
        ] {
            assert_eq!(rebrand(keep), keep);
        }
    }

    #[test]
    fn text_without_the_name_is_borrowed() {
        assert!(matches!(rebrand("Teleport"), std::borrow::Cow::Borrowed(_)));
    }
}
