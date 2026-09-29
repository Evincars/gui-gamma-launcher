// Typed frontend bindings for the Tauri commands in `src-tauri/src/gamma/commands.rs`.
//
// The UI is fully data-driven: call `gammaSchema()` once, render a form per
// command from `commands[].options`, validate with `gammaValidate()`, then
// call `gammaRun({ command, options }, onEvent)`.

import { Channel, invoke } from "@tauri-apps/api/core";

export type GammaOptionType = "path" | "text" | "boolean";
export type GammaCommandGroup = "main" | "tools";

export interface GammaOption {
  /** Key to use inside `GammaRunRequest.options`. */
  key: string;
  /** The underlying CLI flag, e.g. `--anomaly`. */
  flag: string;
  type: GammaOptionType;
  required: boolean;
  description: string;
  /** Example / upstream default ("" if none). */
  placeholder: string;
  /** Initial value ("" if none). */
  default: string;
  /** Longer explanation for a tooltip ("" if none). */
  info: string;
  /** Format of a text option, e.g. `modorganizer-tag`; `null` otherwise. */
  format: string | null;
}

export interface GammaCommand {
  name: string;
  description: string;
  group: GammaCommandGroup;
  options: GammaOption[];
}

export interface GammaSchema {
  binary: string;
  commands: GammaCommand[];
}

export type GammaOptionValue = string | boolean;

export interface GammaRunRequest {
  command: string;
  options: Record<string, GammaOptionValue>;
}

export interface GammaValidation {
  /** Shell-ready command line; `null` when the request is invalid. */
  commandLine: string | null;
  /** Problems per option key. */
  fieldErrors: Record<string, string>;
  /** Problems not tied to a single option. */
  errors: string[];
}

export type GammaRunEvent =
  | { event: "started"; command: string; args: string[] }
  | { event: "stdout"; line: string }
  | { event: "stderr"; line: string }
  | { event: "error"; message: string }
  | {
      event: "finished";
      code: number | null;
      signal: number | null;
      success: boolean;
    };

export interface GammaRunResult {
  code: number | null;
  signal: number | null;
  success: boolean;
}

export type RequirementStatus = "ok" | "warning" | "error";

export interface Requirement {
  id: string;
  label: string;
  status: RequirementStatus;
  detail: string;
  /** How to fix it; only set when not ok. */
  hint: string | null;
}

export interface Requirements {
  /** `gamma-launcher --version`, `null` if it can't start. */
  version: string | null;
  checks: Requirement[];
}

/** Full description of every command and option. */
export function gammaSchema(): Promise<GammaSchema> {
  return invoke<GammaSchema>("gamma_launcher_schema");
}

/** Validate a request; on success includes the exact command line that would run. */
export function gammaValidate(request: GammaRunRequest): Promise<GammaValidation> {
  return invoke<GammaValidation>("gamma_launcher_validate", { request });
}

/** Host dependency checks (launcher, libunrar, 7z, git, certificates, TMPDIR space). */
export function gammaRequirements(): Promise<Requirements> {
  return invoke<Requirements>("gamma_launcher_requirements");
}

/** Kill the currently-running command. Resolves `true` if one was terminated. */
export function gammaCancel(): Promise<boolean> {
  return invoke<boolean>("gamma_launcher_cancel");
}

/** Run a command, streaming stdout/stderr through `onEvent` as it arrives. */
export function gammaRun(
  request: GammaRunRequest,
  onEvent: (event: GammaRunEvent) => void,
): Promise<GammaRunResult> {
  const channel = new Channel<GammaRunEvent>();
  channel.onmessage = onEvent;
  return invoke<GammaRunResult>("gamma_launcher_run", {
    request,
    onEvent: channel,
  });
}
