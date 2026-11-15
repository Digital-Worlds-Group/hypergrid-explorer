# HyperGrid Explorer — ToDo

The changes HyperGrid Explorer still needs, beyond what it inherits from Aurora
Viewer. Aurora's own work list is [TASKS.md](TASKS.md); this file is ours.

Status: ⬜ to do · 🔧 in progress · ✅ done

## Grids

- ⬜ **Grids read from a text file.** The viewer reads its grids from a text
  file at startup instead of having them written in the code. Each line gives a
  grid's **name** and its **URL** (login URI). The grids of this file show as
  the grid choices, with **Second Life and Aditi always at the bottom** of the
  list.
- ⬜ **Grids of the file cannot be removed** with the viewer's tools (grid
  manager, login screen), like Second Life and Aditi today.
- ⬜ **A "Default grid" the user chooses.** Let the user mark one grid as their
  Default grid.
- ⬜ **The grid shown at startup:** the user's Default grid when one is set;
  otherwise the **last grid they logged into**.

## Login screen, per grid

- ⬜ **The grid's splash page as the background.** When the grid shown on the
  login screen (the default grid at startup, or the grid chosen before logging
  in) provides a splash page, use it as the viewer's background; when it has
  none, fall back to HyperGrid Explorer's background picture
  (`assets/branding/hypergrid-explorer-background.png`).
- ⬜ **Grid information API (to be designed and built on the grid side).** When
  the viewer opens on the default grid, and whenever the user changes the grid
  before logging in, contact the grid's API and read:
  - a list of **buttons** — each with its **name** and **URL** — shown at the
    **bottom of the login screen**;
  - the **URL of the news feed** that fills the login screen's News box;
  - the **URL of upcoming events**, if the grid has one;
  - the grid's **current status: Online / Offline**;
  - the **URL to create a new avatar**, for brand-new users.
- ⬜ Viewer side of that API: request it in the background (never blocking the
  login screen), cache the last answer per grid, and show sensible defaults
  when the API is missing or the grid is offline.

## New users

- ⬜ **First start without an avatar.** Someone who downloads the viewer and
  has no avatar yet is offered two choices: **Log in** or **Create a new
  avatar**.
- ⬜ **Create a new avatar flow**, starting from the grid's "create avatar" URL
  (from the grid information API). The flow itself will be defined more
  clearly later.

## Following Aurora Viewer

Aurora Viewer moves fast (about 280 commits in the week of 2026-11-15). Plan:
merge all of Aurora's `main` regularly (weekly) rather than cherry-picking,
cherry-pick only urgent fixes in between, and keep our changes thin (brand
file, our own modules, small hooks into Aurora's code) so merges stay small.

- ⬜ **Sync report script:** fetch `upstream`, list the new Aurora commits since
  our last merge grouped by type (fix / feat / perf) with their PR numbers,
  and flag those touching files we changed (conflict risk).
- ⬜ **Sync branch workflow:** each merge on its own `sync/<date>` branch; build,
  tests and a look at the viewer before it reaches our `main`; then update
  `brand::UPSTREAM_VERSION` / `UPSTREAM_COMMIT` / `UPSTREAM_DATE` and the
  README table.
- ⬜ **Our features on their own branches** (`feat/<name>`), merged into `main`
  when ready, independent of the Aurora syncs.
- ⬜ **First sync:** 4 Aurora fixes were already waiting at the time of the
  rebranding (among them the world not drawing on Macs, #3787).

## Left over from the rebranding (commit 1)

- ⬜ **Public source repository** for HyperGrid Explorer (planned under the
  Digital Worlds Group organisation on GitHub). The GPL asks to offer the
  modified source with the downloads. Then set `brand::GITHUB_REPO` and
  `links::SOURCE` (login screen GitHub button).
- ⬜ **Updates:** the updater is off (it followed Aurora's releases). Publish
  HyperGrid Explorer releases with the `hge-v<version>` tags, point the update
  feed (`aurora-updater/src/feed.rs`) at them, and set `brand::GITHUB_REPO`.
- ⬜ **Crash and bug reports:** posting to GitHub is off (it went to Aurora's
  tracker; the window now opens the website and copies the report). Needs our
  own tracker and GitHub App, or a support page on the website.
- ⬜ **Website pages the viewer links to:** hypergridexplorer.com, a
  **privacy policy** at `/privacy/`, and a support page.
- ⬜ **Help › Wiki** still opens Aurora Viewer's wiki (`links::WIKI`).
- ⬜ **Discord** (login screen button disabled until there is one).
- ⬜ **Windows installer:** the download is a folder (as Aurora's); decide on an
  installer (shortcuts, Start menu, uninstall entry) or a zip.
- ⬜ **Release build** (`--release`) for the public download, and a fixed
  `scripts/build-release.ps1`: it builds Aurora's GitHub `main`, not this fork,
  and on PowerShell 7.6 it fails ("a release tag must start with …") because
  clearing `AURORA_VERSION_TAG` leaves it set to an empty string.
- ⬜ **Linux and macOS packaging** still carry Aurora's identity
  (`packaging/linux/aurora-viewer.desktop`, `packaging/macos/Info.plist`,
  `cli.rs` `DESKTOP_ID`).
- ⬜ **Remaining "Aurora" texts** that are not shown in normal use: the Dev
  Toolbox's copied lines, the crash summary text, the tool name written into
  COLLADA exports. Kept on purpose (data depends on them): the "#Aurora" and
  "Aurora AO" inventory folders, the "Aurora bridge" script, the "Aurora pose"
  file format.
- ⬜ **Developer crash tools:** `scripts/crash-triage.ps1` and its helpers
  look for Aurora's `aurora-viewer` exe and symbols; point them at
  `HyperGridExplorer` (the viewer itself is not affected).
- ⬜ **Agent instructions:** `AGENTS.md` / `CLAUDE.md` still describe Aurora's
  team process (Aurora's GitHub, merge queue, issue board); write our own.
- ⬜ **16 px icon:** the approved 16 × 16 derivative is hard to read (tiny
  "H"); a hand-tuned small size would be for the brand owner to approve.
- ⬜ **Login accessibility button:** Aurora's button opens Preferences on the
  Accessibility page, next to the Preferences gear; keep, replace with a
  dedicated panel, or remove.
