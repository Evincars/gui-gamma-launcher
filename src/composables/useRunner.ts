// Process lifecycle and console output of the launcher.

import { ref } from "vue";

import {
  gammaCancel,
  gammaRun,
  type GammaRunEvent,
  type GammaRunRequest,
  type GammaRunResult,
} from "../lib/gammaLauncher";

export interface ConsoleLine {
  id: number;
  stream: "stdout" | "stderr" | "error" | "meta";
  text: string;
}

/** Oldest lines are dropped past this, keeping the DOM responsive on long installs. */
const MAX_LINES = 5000;

const running = ref(false);
const lastResult = ref<GammaRunResult | null>(null);
const consoleLines = ref<ConsoleLine[]>([]);
let lineSeq = 0;

function pushLine(stream: ConsoleLine["stream"], text: string) {
  consoleLines.value.push({ id: lineSeq++, stream, text });
  const overflow = consoleLines.value.length - MAX_LINES;
  if (overflow > 0) consoleLines.value.splice(0, overflow);
}

function clearConsole() {
  consoleLines.value = [];
}

function onEvent(ev: GammaRunEvent) {
  switch (ev.event) {
    case "stdout":
      pushLine("stdout", ev.line);
      break;
    case "stderr":
      pushLine("stderr", ev.line);
      break;
    case "error":
      pushLine("error", ev.message);
      break;
    case "finished": {
      const signal = ev.signal != null ? `, signal ${ev.signal}` : "";
      pushLine(
        "meta",
        ev.success ? `✔ finished (exit ${ev.code ?? 0})` : `✖ failed (exit ${ev.code ?? "?"}${signal})`,
      );
      break;
    }
  }
}

/** Run an already-validated request; `commandLine` is echoed to the console. */
async function run(request: GammaRunRequest, commandLine: string) {
  if (running.value) return;
  lastResult.value = null;
  running.value = true;
  pushLine("meta", `$ ${commandLine}`);
  try {
    lastResult.value = await gammaRun(request, onEvent);
  } catch (e) {
    pushLine("error", String(e));
  } finally {
    running.value = false;
  }
}

async function cancel() {
  try {
    if (await gammaCancel()) pushLine("meta", "⚠ cancel requested");
  } catch (e) {
    pushLine("error", String(e));
  }
}

export function useRunner() {
  return { running, lastResult, consoleLines, run, cancel, clearConsole };
}
