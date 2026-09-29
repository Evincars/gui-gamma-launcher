// State of the selected command's form: values, validation and error visibility.

import { computed, reactive, ref, watch } from "vue";

import {
  gammaValidate,
  type GammaCommand,
  type GammaOptionValue,
  type GammaRunRequest,
  type GammaValidation,
} from "../lib/gammaLauncher";
import { useSchema } from "./useSchema";
import { isSharedKey, useSharedPaths } from "./useSharedPaths";

type OptionValues = Record<string, GammaOptionValue>;

const VALIDATE_DELAY_MS = 150;
const NO_ISSUES: GammaValidation = { commandLine: null, fieldErrors: {}, errors: [] };

const { schema } = useSchema();
const shared = useSharedPaths();

const selectedName = ref<string | null>(null);
/** Command-specific values: `values[commandName][optionKey]`. Shared paths live in `shared`. */
const values = reactive<Record<string, OptionValues>>({});
const validation = ref<GammaValidation>(NO_ISSUES);
/** Set by a run attempt; reveals errors on still-empty required fields. */
const submitted = ref(false);

const selectedCommand = computed<GammaCommand | null>(
  () => schema.value?.commands.find((c) => c.name === selectedName.value) ?? null,
);

function defaultsFor(cmd: GammaCommand): OptionValues {
  const out: OptionValues = {};
  for (const o of cmd.options) {
    if (!isSharedKey(o.key)) out[o.key] = o.type === "boolean" ? false : o.default;
  }
  return out;
}

function getValue(key: string): GammaOptionValue {
  if (isSharedKey(key)) return shared[key];
  const name = selectedName.value;
  return (name && values[name]?.[key]) ?? "";
}

function setValue(key: string, value: GammaOptionValue) {
  const name = selectedName.value;
  if (isSharedKey(key)) shared[key] = String(value);
  else if (name) values[name][key] = value;
}

const request = computed<GammaRunRequest | null>(() => {
  const cmd = selectedCommand.value;
  if (!cmd) return null;
  const options: OptionValues = {};
  for (const o of cmd.options) options[o.key] = getValue(o.key);
  return { command: cmd.name, options };
});

function select(name: string) {
  const cmd = schema.value?.commands.find((c) => c.name === name);
  if (!cmd) return;
  values[name] ??= defaultsFor(cmd);
  selectedName.value = name;
  submitted.value = false;
  validation.value = NO_ISSUES;
}

function reset() {
  const cmd = selectedCommand.value;
  if (!cmd) return;
  values[cmd.name] = defaultsFor(cmd);
  submitted.value = false;
}

let latest = 0;
async function validate(): Promise<GammaValidation> {
  const req = request.value;
  if (!req) return NO_ISSUES;
  const id = ++latest;
  let result: GammaValidation;
  try {
    result = await gammaValidate(req);
  } catch (e) {
    result = { ...NO_ISSUES, errors: [String(e)] };
  }
  // Ignore responses that were overtaken by a newer edit.
  if (id === latest) validation.value = result;
  return result;
}

let timer: ReturnType<typeof setTimeout> | undefined;
watch(request, () => {
  clearTimeout(timer);
  timer = setTimeout(validate, VALIDATE_DELAY_MS);
});

watch(
  schema,
  (s) => {
    if (s?.commands.length && !selectedName.value) select(s.commands[0].name);
  },
  { immediate: true },
);

/** Mark the form as submitted and validate immediately (no debounce). */
function submit(): Promise<GammaValidation> {
  clearTimeout(timer);
  submitted.value = true;
  return validate();
}

/** Error to show for `key`: always once submitted, otherwise only for filled-in fields. */
function visibleError(key: string): string | undefined {
  const message = validation.value.fieldErrors[key];
  if (!message) return undefined;
  const value = getValue(key);
  const filled = typeof value === "string" && value.trim() !== "";
  return submitted.value || filled ? message : undefined;
}

const issueCount = computed(
  () => Object.keys(validation.value.fieldErrors).length + validation.value.errors.length,
);

export function useCommandForm() {
  return {
    selectedName,
    selectedCommand,
    request,
    validation,
    submitted,
    issueCount,
    getValue,
    setValue,
    select,
    reset,
    validate,
    submit,
    visibleError,
  };
}
