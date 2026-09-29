//! Serializable description of the CLI, sent to the UI so forms are generated
//! from the spec instead of being hardcoded.

use serde::Serialize;

use super::sidecar;
use super::spec::{CmdGroup, CmdSpec, OptSpec, COMMANDS};

#[derive(Serialize)]
pub struct Schema {
    binary: &'static str,
    commands: Vec<CommandDto>,
}

#[derive(Serialize)]
struct CommandDto {
    name: &'static str,
    description: &'static str,
    group: CmdGroup,
    options: Vec<OptionDto>,
}

#[derive(Serialize)]
struct OptionDto {
    key: &'static str,
    flag: &'static str,
    #[serde(rename = "type")]
    kind: &'static str,
    required: bool,
    description: &'static str,
    placeholder: &'static str,
    default: &'static str,
    info: &'static str,
    /// Format of a text option (see `TextRule::id`), `null` otherwise.
    format: Option<&'static str>,
}

impl From<&OptSpec> for OptionDto {
    fn from(o: &OptSpec) -> Self {
        Self {
            key: o.key,
            flag: o.flag,
            kind: o.kind.ui_type(),
            required: o.required,
            description: o.help,
            placeholder: o.placeholder,
            default: o.default,
            info: o.info,
            format: o.kind.text_format(),
        }
    }
}

impl From<&CmdSpec> for CommandDto {
    fn from(c: &CmdSpec) -> Self {
        Self {
            name: c.name,
            description: c.help,
            group: c.group,
            options: c.options.iter().map(OptionDto::from).collect(),
        }
    }
}

pub(crate) fn schema() -> Schema {
    Schema {
        binary: sidecar::NAME,
        commands: COMMANDS.iter().map(CommandDto::from).collect(),
    }
}
