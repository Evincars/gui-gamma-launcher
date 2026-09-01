<script setup lang="ts">
import { ref } from "vue";

import BaseButton from "./common/BaseButton.vue";
import { useGammaLauncher } from "../composables/useGammaLauncher";

const { previewText, previewError } = useGammaLauncher();

const copied = ref(false);

async function copy() {
  if (!previewText.value) return;
  try {
    await navigator.clipboard.writeText(previewText.value);
    copied.value = true;
    setTimeout(() => (copied.value = false), 1200);
  } catch {
    /* clipboard unavailable — ignore */
  }
}
</script>

<template>
  <section class="preview">
    <div class="preview__head">
      <span class="preview__title">Command line</span>
      <BaseButton :disabled="!previewText" @click="copy">
        {{ copied ? "Copied" : "Copy" }}
      </BaseButton>
    </div>

    <p v-if="previewError" class="preview__error">{{ previewError }}</p>
    <pre v-else class="preview__code"><code>{{ previewText || "…" }}</code></pre>
  </section>
</template>

<style scoped>
.preview {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preview__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.preview__title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-faint);
}

.preview__code {
  margin: 0;
  padding: 12px 14px;
  border-radius: var(--radius-sm);
  background: var(--bg-inset);
  border: 1px solid var(--panel-border);
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--accent);
  white-space: pre-wrap;
  word-break: break-all;
}

.preview__error {
  margin: 0;
  padding: 12px 14px;
  border-radius: var(--radius-sm);
  background: rgba(240, 96, 60, 0.1);
  border: 1px solid var(--danger);
  color: var(--danger-hover);
  font-size: 12px;
}
</style>
