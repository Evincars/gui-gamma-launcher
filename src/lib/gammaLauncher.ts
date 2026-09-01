// Typed frontend bindings for the gamma-launcher mapper in `src-tauri/src/lib.rs`.
//
// The UI is fully data-driven: call `gammaSchema()` once, render a form per
// command from `commands[].options`, collect values into a plain object, then
// call `gammaRun({ command, options }, onEvent)`.

import { Channel, invoke } from "@tauri-apps/api/core";

export type GammaOptionType = "path" | "text" | "boolean";

export interface GammaOption {
  /** Key to use inside `GammaRunRequest.options`. */
  key: string;
  /** The underlying CLI flag, e.g. `--anomaly`. */
  flag: string;
  type: GammaOptionType;
  required: boolean;
  description: string;
}

export interface GammaCommand {
  name: string;
  description: string;
  options: GammaOption[];
}

export interface GammaSchema {
  binary: string;
  commands: GammaCommand[];
}

export type GammaOptionValue = string | boolean;

export interface GammaRunRequest {
  command: string;
  options?: Record<string, GammaOptionValue>;
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
  stdout: string;
  stderr: string;
}

/** Full description of every command and option. */
export function gammaSchema(): Promise<GammaSchema> {
  return invoke<GammaSchema>("gamma_launcher_schema");
}

/** Human-readable command line that `gammaRun` would execute (also validates). */
export function gammaPreview(request: GammaRunRequest): Promise<string> {
  return invoke<string>("gamma_launcher_preview", { request });
}

/** Raw argv form, e.g. `["full-install", "--anomaly", "/path", ...]`. */
export function gammaArgs(request: GammaRunRequest): Promise<string[]> {
  return invoke<string[]>("gamma_launcher_args", { request });
}

/** `gamma-launcher --version`. */
export function gammaVersion(): Promise<string> {
  return invoke<string>("gamma_launcher_version");
}

/** Kill the currently-running command. Resolves `true` if one was terminated. */
export function gammaCancel(): Promise<boolean> {
  return invoke<boolean>("gamma_launcher_cancel");
}

/**
 * Run a command, streaming stdout/stderr through `onEvent` as it arrives.
 * Resolves with the aggregated output and exit status when the process ends.
 */
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
