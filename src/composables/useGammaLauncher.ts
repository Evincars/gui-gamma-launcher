// Central, app-wide state for the launcher UI. Module-scoped refs make this a
// singleton: every component that calls `useGammaLauncher()` shares one store.

import { computed, reactive, ref, watch } from "vue";

import {
  gammaCancel,
  gammaPreview,
  gammaRun,
  gammaSchema,
  gammaVersion,
  type GammaCommand,
  type GammaOptionValue,
  type GammaRunResult,
  type GammaSchema,
} from "../lib/gammaLauncher";

export interface ConsoleLine {
  id: number;
  stream: "stdout" | "stderr" | "meta";
  text: string;
}

type OptionValues = Record<string, GammaOptionValue>;

const schema = ref<GammaSchema | null>(null);
const schemaError = ref<string | null>(null);
const version = ref<string | null>(null);
const loading = ref(true);

const selectedName = ref<string | null>(null);
/** `values[commandName][optionKey]` */
const values = reactive<Record<string, OptionValues>>({});

const previewText = ref("");
const previewError = ref<string | null>(null);

const running = ref(false);
const lastResult = ref<GammaRunResult | null>(null);
const consoleLines = ref<ConsoleLine[]>([]);
let lineSeq = 0;

const selectedCommand = computed<GammaCommand | null>(() => {
  if (!schema.value || !selectedName.value) return null;
  return schema.value.commands.find((c) => c.name === selectedName.value) ?? null;
});

const currentValues = computed<OptionValues>(() => {
  const name = selectedName.value;
  return name ? values[name] ?? {} : {};
});

const canRun = computed(
  () => !running.value && !loading.value && !!selectedName.value && !previewError.value,
);

function defaultsFor(cmd: GammaCommand): OptionValues {
  const out: OptionValues = {};
  for (const o of cmd.options) out[o.key] = o.type === "boolean" ? false : "";
  return out;
}

function ensureValues(cmd: GammaCommand) {
  if (!values[cmd.name]) values[cmd.name] = defaultsFor(cmd);
}

function selectCommand(name: string) {
  selectedName.value = name;
  const cmd = schema.value?.commands.find((c) => c.name === name);
  if (cmd) ensureValues(cmd);
}

function resetCommand() {
  const cmd = selectedCommand.value;
  if (cmd) values[cmd.name] = defaultsFor(cmd);
}

function pushLine(stream: ConsoleLine["stream"], text: string) {
  consoleLines.value.push({ id: lineSeq++, stream, text });
}

function clearConsole() {
  consoleLines.value = [];
}

async function load() {
  loading.value = true;
  schemaError.value = null;
  try {
    const loaded = await gammaSchema();
    schema.value = loaded;
    for (const c of loaded.commands) ensureValues(c);
    if (!selectedName.value && loaded.commands.length) {
      selectCommand(loaded.commands[0].name);
    }
  } catch (e) {
    schemaError.value = String(e);
  } finally {
    loading.value = false;
  }

  gammaVersion()
    .then((v) => (version.value = v))
    .catch(() => (version.value = null));
}

// Live, debounced preview — also serves as form validation.
let previewTimer: ReturnType<typeof setTimeout> | undefined;
watch(
  [selectedName, currentValues],
  () => {
    const name = selectedName.value;
    if (!name) {
      previewText.value = "";
      previewError.value = null;
      return;
    }
    clearTimeout(previewTimer);
    previewTimer = setTimeout(async () => {
      try {
        previewText.value = await gammaPreview({
          command: name,
          options: { ...values[name] },
        });
        previewError.value = null;
      } catch (e) {
        previewText.value = "";
        previewError.value = String(e);
      }
    }, 120);
  },
  { deep: true, immediate: true },
);

async function run() {
  const name = selectedName.value;
  if (!name || running.value) return;

  lastResult.value = null;
  running.value = true;
  pushLine("meta", `$ ${previewText.value || name}`);

  try {
    lastResult.value = await gammaRun(
      { command: name, options: { ...values[name] } },
      (ev) => {
        switch (ev.event) {
          case "stdout":
            pushLine("stdout", ev.line);
            break;
          case "stderr":
            pushLine("stderr", ev.line);
            break;
          case "error":
            pushLine("stderr", `[error] ${ev.message}`);
            break;
          case "finished":
            pushLine(
              "meta",
              ev.success
                ? `✔ finished (exit ${ev.code ?? 0})`
                : `✖ failed (exit ${ev.code ?? "?"}${
                    ev.signal != null ? `, signal ${ev.signal}` : ""
                  })`,
            );
            break;
        }
      },
    );
  } catch (e) {
    pushLine("stderr", String(e));
  } finally {
    running.value = false;
  }
}

async function cancel() {
  try {
    if (await gammaCancel()) pushLine("meta", "⚠ cancel requested");
  } catch (e) {
    pushLine("stderr", String(e));
  }
}

export function useGammaLauncher() {
  return {
    // state
    schema,
    schemaError,
    version,
    loading,
    selectedName,
    selectedCommand,
    currentValues,
    values,
    previewText,
    previewError,
    running,
    canRun,
    lastResult,
    consoleLines,
    // actions
    load,
    selectCommand,
    resetCommand,
    run,
    cancel,
    clearConsole,
  };
}
