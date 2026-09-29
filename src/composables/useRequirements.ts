// Host dependency checks + launcher version. App-wide singleton.

import { computed, ref } from "vue";

import { gammaRequirements, type Requirements } from "../lib/gammaLauncher";

const result = ref<Requirements | null>(null);
const checking = ref(false);
const checkError = ref<string | null>(null);

async function check() {
  checking.value = true;
  checkError.value = null;
  try {
    result.value = await gammaRequirements();
  } catch (e) {
    checkError.value = String(e);
  } finally {
    checking.value = false;
  }
}

const checks = computed(() => result.value?.checks ?? []);
const version = computed(() => result.value?.version ?? null);
const errorCount = computed(() => checks.value.filter((c) => c.status === "error").length);
const warningCount = computed(() => checks.value.filter((c) => c.status === "warning").length);

export function useRequirements() {
  return { checks, version, checking, checkError, errorCount, warningCount, check };
}
