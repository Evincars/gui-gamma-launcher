//! Static description of the `gamma-launcher` v3.1 CLI surface.
//!
//! Every subcommand and option lives in [`COMMANDS`] — the single source of
//! truth consumed by the mapper, the validator and the schema sent to the UI.
//!
//! Reference (upstream `launcher/commands/*.py` at tag `v3.1`):
//! ```text
//! anomaly-install    --anomaly (req) [--cache-directory] [--anomaly-skip-verify] [--anomaly-purge-cache]
//! check-anomaly      --anomaly (req)
//! check-md5          --gamma (req) [--update-cache] [--remove-unused]
//! full-install       --anomaly (req) --gamma (req) [--cache-directory] [--anomaly-skip-verify]
//!                    [--anomaly-purge-cache] [--gamma-no-mod-organizer]
//!                    [--gamma-set-mod-organizer-version V] [--custom-gamma-definition V]
//!                    [--custom-gamma-repository V] [--no-def-update] [--no-anomaly-patch]
//!                    [--preserve-user-config]
//! gamma-setup        --gamma (req) [--cache-directory] [--gamma-no-mod-organizer]
//!                    [--gamma-set-mod-organizer-version V]
//! remove-reshade     --anomaly (req)
//! purge-shader-cache --anomaly (req)
//! switch-keymap      --anomaly (req) [--to-dvorak]
//! test-mod-maker     --gamma (req)
//! usvfs-workaround   --anomaly (req) --gamma (req) --final (req)
//! ```

use serde::Serialize;

/// Sidebar section a command belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CmdGroup {
    /// Install / verify workflow, shown first.
    Main,
    Tools,
}

/// Expectation on a directory option, checked before the launcher runs.
#[derive(Clone, Copy)]
pub(crate) enum PathRule {
    /// The launcher creates the directory if it is missing.
    Create,
    /// Must be an existing directory containing every listed entry.
    Existing(&'static [&'static str]),
}

/// Format a free-text option must follow.
#[derive(Clone, Copy)]
pub(crate) enum TextRule {
    /// ModOrganizer GitHub release tag, e.g. `v2.5.2`.
    ModOrganizerTag,
    /// Git commit, tag or branch (used in a GitHub archive URL).
    GitRevision,
    /// `owner/repository` on GitHub.
    GithubRepo,
}

impl TextRule {
    /// Identifier the UI uses to attach format-specific helpers (e.g. suggestions).
    pub(crate) fn id(self) -> &'static str {
        match self {
            TextRule::ModOrganizerTag => "modorganizer-tag",
            TextRule::GitRevision => "git-revision",
            TextRule::GithubRepo => "github-repo",
        }
    }
}

/// Kind of value an option carries.
#[derive(Clone, Copy)]
pub(crate) enum OptKind {
    Path(PathRule),
    Text(TextRule),
    /// On/off switch — the flag is passed with no value when enabled.
    Bool,
}

impl OptKind {
    /// Control type the UI should render.
    pub(crate) fn ui_type(self) -> &'static str {
        match self {
            OptKind::Path(_) => "path",
            OptKind::Text(_) => "text",
            OptKind::Bool => "boolean",
        }
    }

    pub(crate) fn text_format(self) -> Option<&'static str> {
        match self {
            OptKind::Text(rule) => Some(rule.id()),
            _ => None,
        }
    }
}

/// A single option of a command.
#[derive(Clone, Copy)]
pub(crate) struct OptSpec {
    /// Key the frontend uses in the `options` map (camelCase).
    pub(crate) key: &'static str,
    /// The actual CLI flag, e.g. `--anomaly`.
    pub(crate) flag: &'static str,
    pub(crate) kind: OptKind,
    pub(crate) required: bool,
    pub(crate) help: &'static str,
    /// Example shown in an empty input.
    pub(crate) placeholder: &'static str,
    /// Initial value in the UI ("" = empty).
    pub(crate) default: &'static str,
    /// Longer explanation shown as a tooltip ("" = none).
    pub(crate) info: &'static str,
}

/// A subcommand and its full option set.
pub(crate) struct CmdSpec {
    pub(crate) name: &'static str,
    pub(crate) help: &'static str,
    pub(crate) group: CmdGroup,
    pub(crate) options: &'static [OptSpec],
}

// -- Shared option definitions -------------------------------------------------

const fn anomaly(rule: PathRule) -> OptSpec {
    OptSpec {
        key: "anomaly",
        flag: "--anomaly",
        kind: OptKind::Path(rule),
        required: true,
        help: "Path to ANOMALY directory",
        placeholder: "",
        default: "",
        info: "",
    }
}

const fn gamma(rule: PathRule) -> OptSpec {
    OptSpec {
        key: "gamma",
        flag: "--gamma",
        kind: OptKind::Path(rule),
        required: true,
        help: "Path to GAMMA directory",
        placeholder: "",
        default: "",
        info: "",
    }
}

const fn switch(key: &'static str, flag: &'static str, help: &'static str) -> OptSpec {
    OptSpec {
        key,
        flag,
        kind: OptKind::Bool,
        required: false,
        help,
        placeholder: "",
        default: "",
        info: "",
    }
}

/// Anomaly dir that must already hold an installation.
const ANOMALY_INSTALLED: OptSpec = anomaly(PathRule::Existing(&["bin"]));

const CACHE_DIRECTORY: OptSpec = OptSpec {
    key: "cacheDirectory",
    flag: "--cache-directory",
    kind: OptKind::Path(PathRule::Create),
    required: false,
    help: "Path to cache directory",
    placeholder: "",
    default: "",
    info: "",
};
const ANOMALY_SKIP_VERIFY: OptSpec = switch(
    "anomalySkipVerify",
    "--anomaly-skip-verify",
    "Skip installation verification",
);
const ANOMALY_PURGE_CACHE: OptSpec = switch(
    "anomalyPurgeCache",
    "--anomaly-purge-cache",
    "Do not keep 7z archives",
);
const GAMMA_NO_MOD_ORGANIZER: OptSpec = switch(
    "gammaNoModOrganizer",
    "--gamma-no-mod-organizer",
    "Skip ModOrganizer installation",
);
const GAMMA_SET_MOD_ORGANIZER_VERSION: OptSpec = OptSpec {
    key: "gammaSetModOrganizerVersion",
    flag: "--gamma-set-mod-organizer-version",
    kind: OptKind::Text(TextRule::ModOrganizerTag),
    required: false,
    help: "Set ModOrganizer Version (have to match github tags). Not sure? Leave it empty — gamma-launcher then uses v2.5.2.",
    placeholder: "v2.5.2",
    default: "",
    info: "Release tag of ModOrganizer2 (github.com/ModOrganizer2/modorganizer/releases). \
The launcher downloads releases/download/<tag>/Mod.Organizer-<version>.7z, so only tags \
that publish that archive work.\n\n\
v2.5.2 is gamma-launcher's built-in default and the archive `check-md5 --remove-unused` keeps.\n\
Ignored when \"Gamma no mod organizer\" is on.",
};

/// Every command supported by the bundled `gamma-launcher` binary, in UI order.
pub(crate) static COMMANDS: &[CmdSpec] = &[
    CmdSpec {
        name: "full-install",
        help: "Complete install of S.T.A.L.K.E.R.: G.A.M.M.A.",
        group: CmdGroup::Main,
        options: &[
            anomaly(PathRule::Create),
            gamma(PathRule::Create),
            CACHE_DIRECTORY,
            ANOMALY_SKIP_VERIFY,
            ANOMALY_PURGE_CACHE,
            GAMMA_NO_MOD_ORGANIZER,
            GAMMA_SET_MOD_ORGANIZER_VERSION,
            OptSpec {
                key: "customGammaDefinition",
                flag: "--custom-gamma-definition",
                kind: OptKind::Text(TextRule::GitRevision),
                required: false,
                help: "Set a custom revision for S.T.A.L.K.E.R.: G.A.M.M.A. Advanced setting — leave empty for a regular install.",
                placeholder: "commit, tag or branch",
                default: "",
                info: "Pins the G.A.M.M.A. modpack definition to a specific revision instead of the latest one.\n\n\
• Downloads https://github.com/<repository>/archive/<revision>.zip — a commit hash, tag or branch.\n\
• Writes \"Custom: <revision>\" to <GAMMA>/.Grok's Modpack Installer/revision.txt; later \
full-installs without this option then skip definition updates. Delete that file (or pass \
another revision) to go back to normal updates.\n\
• Ignored when \"No def update\" is on.",
            },
            OptSpec {
                key: "customGammaRepository",
                flag: "--custom-gamma-repository",
                kind: OptKind::Text(TextRule::GithubRepo),
                required: false,
                help: "Set a custom repository for S.T.A.L.K.E.R.: G.A.M.M.A. Advanced setting — leave empty for a regular install.",
                placeholder: "Grokitach/Stalker_GAMMA",
                default: "",
                info: "GitHub owner/repository the G.A.M.M.A. definition is fetched from \
(default Grokitach/Stalker_GAMMA), e.g. a fork.\n\n\
Normal updates track its `main` branch; with \"Custom gamma definition\" the given revision \
of this repository is used instead. Ignored when \"No def update\" is on.",
            },
            switch(
                "noDefUpdate",
                "--no-def-update",
                "Do not update S.T.A.L.K.E.R.: G.A.M.M.A. definition",
            ),
            switch(
                "noAnomalyPatch",
                "--no-anomaly-patch",
                "Do not patch Anomaly directory",
            ),
            switch(
                "preserveUserConfig",
                "--preserve-user-config",
                "Do not overwrite user configuration when patching Anomaly directory",
            ),
        ],
    },
    CmdSpec {
        name: "anomaly-install",
        help: "Installation of S.T.A.L.K.E.R.: Anomaly",
        group: CmdGroup::Main,
        options: &[
            anomaly(PathRule::Create),
            CACHE_DIRECTORY,
            ANOMALY_SKIP_VERIFY,
            ANOMALY_PURGE_CACHE,
        ],
    },
    CmdSpec {
        name: "check-anomaly",
        help: "Check Anomaly installation",
        group: CmdGroup::Main,
        options: &[anomaly(PathRule::Existing(&["tools/checksums.md5"]))],
    },
    CmdSpec {
        name: "check-md5",
        help: "Check MD5 hash for all addons",
        group: CmdGroup::Main,
        options: &[
            gamma(PathRule::Existing(&[
                ".Grok's Modpack Installer/G.A.M.M.A/modpack_data",
            ])),
            switch(
                "updateCache",
                "--update-cache",
                "Update download cache if file is missing or MD5 do not match",
            ),
            switch(
                "removeUnused",
                "--remove-unused",
                "After hash checks, remove unused archive in download directory",
            ),
        ],
    },
    CmdSpec {
        name: "gamma-setup",
        help: "Preliminary setup for S.T.A.L.K.E.R.: G.A.M.M.A.",
        group: CmdGroup::Tools,
        options: &[
            gamma(PathRule::Create),
            CACHE_DIRECTORY,
            GAMMA_NO_MOD_ORGANIZER,
            GAMMA_SET_MOD_ORGANIZER_VERSION,
        ],
    },
    CmdSpec {
        name: "remove-reshade",
        help: "Remove ReShade from Anomaly bin",
        group: CmdGroup::Tools,
        options: &[ANOMALY_INSTALLED],
    },
    CmdSpec {
        name: "purge-shader-cache",
        help: "Purge Anomaly shader cache",
        group: CmdGroup::Tools,
        options: &[ANOMALY_INSTALLED],
    },
    CmdSpec {
        name: "switch-keymap",
        help: "Switch keymap of user.ltx from QWERTY to AZERTY layout",
        group: CmdGroup::Tools,
        options: &[
            anomaly(PathRule::Existing(&["appdata/user.ltx"])),
            switch("toDvorak", "--to-dvorak", "Use DVORAK instead of AZERTY"),
        ],
    },
    CmdSpec {
        name: "test-mod-maker",
        help: "Testing mod maker directives (disabled upstream in v3.1)",
        group: CmdGroup::Tools,
        options: &[gamma(PathRule::Existing(&[]))],
    },
    CmdSpec {
        name: "usvfs-workaround",
        help: "Workaround to use wine without ModOrganizer (& UserSpace Virtual FileSystem)",
        group: CmdGroup::Tools,
        options: &[
            ANOMALY_INSTALLED,
            gamma(PathRule::Existing(&["mods", "profiles/G.A.M.M.A/modlist.txt"])),
            OptSpec {
                key: "final",
                flag: "--final",
                kind: OptKind::Path(PathRule::Create),
                required: true,
                help: "Path to final install directory",
                placeholder: "",
                default: "",
                info: "",
            },
        ],
    },
];

/// Look up a command spec by CLI name.
pub(crate) fn find_command(name: &str) -> Option<&'static CmdSpec> {
    COMMANDS.iter().find(|c| c.name == name)
}
