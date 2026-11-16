//! Project links shown on the login screen. Empty = not configured yet: the
//! button is shown disabled and nothing is fetched. The README header carries
//! the same Discord and website links: change them there too.

/// Source code of Aurora Viewer, the upstream HyperGrid Explorer is based on
/// (About › licence notices: "Source code").
pub const GITHUB: &str = "https://github.com/Aurora-Viewer/Aurora-Viewer";
/// HyperGrid Explorer's own source repository, for the login screen's GitHub
/// button: the GPL asks to offer the modified source with the downloads.
pub const SOURCE: &str = "https://github.com/Digital-Worlds-Group/hypergrid-explorer";
/// Issue template chooser, opened in the browser by "Report a bug" (login
/// screen, top bar, Help menu) and by the crash window.
/// HyperGrid Explorer: the website until the fork has its own tracker (never
/// Aurora Viewer's).
pub const NEW_ISSUE: &str = aurora_version::brand::WEBSITE;
/// The project's wiki, generated from the guides of `docs/`
/// (`scripts/build-wiki.ps1`): Help › Aurora Viewer wiki.
pub const WIKI: &str = "https://github.com/Aurora-Viewer/Aurora-Viewer/wiki";
/// Community Discord invite.
/// HyperGrid Explorer: none yet (Aurora Viewer's server is not ours).
pub const DISCORD: &str = "";
/// Project website.
pub const WEBSITE: &str = aurora_version::brand::WEBSITE;
/// The privacy policy on the website (Preferences › Privacy › Your data).
pub const PRIVACY_POLICY: &str = "https://hypergridexplorer.com/privacy/";
/// News feed: this folder of `main`, hand-edited Markdown files, one
/// `<language>.md` per UI language (`news/` at the repository root). The
/// viewer keeps what it read in its cache as a JSON array of
/// `{"date", "title", "summary", "url"}`.
/// HyperGrid Explorer: empty (no news shown) until the fork publishes its
/// own; Aurora Viewer's news are about Aurora's releases.
pub const NEWS_FEED: &str = "";
/// Files of the source repository as GitHub serves them unrendered; a branch
/// name and a path follow.
pub const RAW_FILES: &str = "https://raw.githubusercontent.com/Aurora-Viewer/Aurora-Viewer/";
