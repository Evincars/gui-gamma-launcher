// Command schema + binary health. Module-scoped refs make this an app-wide singleton.

import { ref } from "vue";

import { gammaSchema, gammaVersion, type GammaSchema } from "../lib/gammaLauncher";

const schema = ref<GammaSchema | null>(null);
const schemaError = ref<string | null>(null);
const loading = ref(true);
const version = ref<string | null>(null);
/** Why the bundled binary can't start (e.g. missing libunrar), if it can't. */
const binaryError = ref<string | null>(null);

async function checkBinary() {
  try {
    version.value = await gammaVersion();
    binaryError.value = null;
  } catch (e) {
    version.value = null;
    binaryError.value = String(e);
  }
}

async function load() {
  loading.value = true;
  schemaError.value = null;
  try {
    schema.value = await gammaSchema();
  } catch (e) {
    schemaError.value = String(e);
  } finally {
    loading.value = false;
  }
  void checkBinary();
}

export function useSchema() {
  return { schema, schemaError, loading, version, binaryError, load, checkBinary };
}
