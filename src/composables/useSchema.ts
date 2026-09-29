// Command schema. Module-scoped refs make this an app-wide singleton.

import { ref } from "vue";

import { gammaSchema, type GammaSchema } from "../lib/gammaLauncher";

const schema = ref<GammaSchema | null>(null);
const schemaError = ref<string | null>(null);
const loading = ref(true);

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
}

export function useSchema() {
  return { schema, schemaError, loading, load };
}
