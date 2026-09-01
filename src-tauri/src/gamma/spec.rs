//! Static description of the `gamma-launcher` v3.1 CLI surface.
//!
//! Every subcommand and option lives in [`COMMANDS`] — the single source of
//! truth consumed by both the argument mapper and the schema exposed to the UI.
//!
//! Reference (`gamma-launcher-v3.1 <cmd> --help`):
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

/// Kind of value an option carries, and how the UI should render it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum OptKind {
    /// Filesystem path — rendered as a folder picker.
    Path,
    /// Free text value (git tag, repo URL, …).
    Text,
    /// On/off switch — the flag is passed with no value when enabled.
    Bool,
}

impl OptKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            OptKind::Path => "path",
            OptKind::Text => "text",
            OptKind::Bool => "boolean",
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
}

/// A subcommand and its full option set.
pub(crate) struct CmdSpec {
    pub(crate) name: &'static str,
    pub(crate) help: &'static str,
    pub(crate) options: &'static [OptSpec],
}

// -- Shared option definitions -------------------------------------------------

const ANOMALY: OptSpec = OptSpec {
    key: "anomaly",
    flag: "--anomaly",
    kind: OptKind::Path,
    required: true,
    help: "Path to ANOMALY directory",
};
const GAMMA: OptSpec = OptSpec {
    key: "gamma",
    flag: "--gamma",
    kind: OptKind::Path,
    required: true,
    help: "Path to GAMMA directory",
};
const CACHE_DIRECTORY: OptSpec = OptSpec {
    key: "cacheDirectory",
    flag: "--cache-directory",
    kind: OptKind::Path,
    required: false,
    help: "Path to cache directory",
};
const ANOMALY_SKIP_VERIFY: OptSpec = OptSpec {
    key: "anomalySkipVerify",
    flag: "--anomaly-skip-verify",
    kind: OptKind::Bool,
    required: false,
    help: "Skip installation verification",
};
const ANOMALY_PURGE_CACHE: OptSpec = OptSpec {
    key: "anomalyPurgeCache",
    flag: "--anomaly-purge-cache",
    kind: OptKind::Bool,
    required: false,
    help: "Do not keep 7z archives",
};
const GAMMA_NO_MOD_ORGANIZER: OptSpec = OptSpec {
    key: "gammaNoModOrganizer",
    flag: "--gamma-no-mod-organizer",
    kind: OptKind::Bool,
    required: false,
    help: "Skip ModOrganizer installation",
};
const GAMMA_SET_MOD_ORGANIZER_VERSION: OptSpec = OptSpec {
    key: "gammaSetModOrganizerVersion",
    flag: "--gamma-set-mod-organizer-version",
    kind: OptKind::Text,
    required: false,
    help: "Set ModOrganizer Version (have to match github tags)",
};

/// Every command supported by the bundled `gamma-launcher` binary.
pub(crate) static COMMANDS: &[CmdSpec] = &[
    CmdSpec {
        name: "anomaly-install",
        help: "Installation of S.T.A.L.K.E.R.: Anomaly",
        options: &[
            ANOMALY,
            CACHE_DIRECTORY,
            ANOMALY_SKIP_VERIFY,
            ANOMALY_PURGE_CACHE,
        ],
    },
    CmdSpec {
        name: "check-anomaly",
        help: "Check Anomaly installation",
        options: &[ANOMALY],
    },
    CmdSpec {
        name: "check-md5",
        help: "Check MD5 hash for all addons",
        options: &[
            GAMMA,
            OptSpec {
                key: "updateCache",
                flag: "--update-cache",
                kind: OptKind::Bool,
                required: false,
                help: "Update download cache if file is missing or MD5 do not match",
            },
            OptSpec {
                key: "removeUnused",
                flag: "--remove-unused",
                kind: OptKind::Bool,
                required: false,
                help: "After hash checks, remove unused archive in download directory",
            },
        ],
    },
    CmdSpec {
        name: "full-install",
        help: "Complete install of S.T.A.L.K.E.R.: G.A.M.M.A.",
        options: &[
            ANOMALY,
            GAMMA,
            CACHE_DIRECTORY,
            ANOMALY_SKIP_VERIFY,
            ANOMALY_PURGE_CACHE,
            GAMMA_NO_MOD_ORGANIZER,
            GAMMA_SET_MOD_ORGANIZER_VERSION,
            OptSpec {
                key: "customGammaDefinition",
                flag: "--custom-gamma-definition",
                kind: OptKind::Text,
                required: false,
                help: "Set a custom revision for S.T.A.L.K.E.R.: G.A.M.M.A.",
            },
            OptSpec {
                key: "customGammaRepository",
                flag: "--custom-gamma-repository",
                kind: OptKind::Text,
                required: false,
                help: "Set a custom repository for S.T.A.L.K.E.R.: G.A.M.M.A.",
            },
            OptSpec {
                key: "noDefUpdate",
                flag: "--no-def-update",
                kind: OptKind::Bool,
                required: false,
                help: "Do not update S.T.A.L.K.E.R.: G.A.M.M.A. definition",
            },
            OptSpec {
                key: "noAnomalyPatch",
                flag: "--no-anomaly-patch",
                kind: OptKind::Bool,
                required: false,
                help: "Do not patch Anomaly directory",
            },
            OptSpec {
                key: "preserveUserConfig",
                flag: "--preserve-user-config",
                kind: OptKind::Bool,
                required: false,
                help: "Do not overwrite user configuration when patching Anomaly directory",
            },
        ],
    },
    CmdSpec {
        name: "gamma-setup",
        help: "Preliminary setup for S.T.A.L.K.E.R.: G.A.M.M.A.",
        options: &[
            GAMMA,
            CACHE_DIRECTORY,
            GAMMA_NO_MOD_ORGANIZER,
            GAMMA_SET_MOD_ORGANIZER_VERSION,
        ],
    },
    CmdSpec {
        name: "remove-reshade",
        help: "Remove ReShade from Anomaly bin",
        options: &[ANOMALY],
    },
    CmdSpec {
        name: "purge-shader-cache",
        help: "Purge Anomaly shader cache",
        options: &[ANOMALY],
    },
    CmdSpec {
        name: "switch-keymap",
        help: "Switch keymap of user.ltx from QWERTY to AZERTY layout",
        options: &[
            ANOMALY,
            OptSpec {
                key: "toDvorak",
                flag: "--to-dvorak",
                kind: OptKind::Bool,
                required: false,
                help: "Use DVORAK instead of AZERTY",
            },
        ],
    },
    CmdSpec {
        name: "test-mod-maker",
        help: "Testing mod maker directives",
        options: &[GAMMA],
    },
    CmdSpec {
        name: "usvfs-workaround",
        help: "Workaround to use wine without ModOrganizer (& UserSpace Virtual FileSystem)",
        options: &[
            ANOMALY,
            GAMMA,
            OptSpec {
                key: "final",
                flag: "--final",
                kind: OptKind::Path,
                required: true,
                help: "Path to final install directory",
            },
        ],
    },
];

/// Look up a command spec by CLI name.
pub(crate) fn find_command(name: &str) -> Result<&'static CmdSpec, String> {
    COMMANDS
        .iter()
        .find(|c| c.name == name)
        .ok_or_else(|| format!("Unknown gamma-launcher command: `{name}`"))
}
