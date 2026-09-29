<script setup lang="ts">
import { computed, ref } from "vue";

import AlertBar from "./common/AlertBar.vue";
import BaseButton from "./common/BaseButton.vue";
import PanelHeader from "./common/PanelHeader.vue";
import { useCommandForm } from "../composables/useCommandForm";

const { validation, submitted, issueCount } = useCommandForm();

const commandLine = computed(() => validation.value.commandLine);
const showIssues = computed(
  () => !commandLine.value && (submitted.value || validation.value.errors.length > 0),
);

const copied = ref(false);

async function copy() {
  if (!commandLine.value) return;
  try {
    await navigator.clipboard.writeText(commandLine.value);
    copied.value = true;
    setTimeout(() => (copied.value = false), 1200);
  } catch {
    // Clipboard unavailable — nothing useful to report.
  }
}
</script>

<template>
  <section class="preview">
    <PanelHeader title="Command line">
      <BaseButton :disabled="!commandLine" @click="copy">
        {{ copied ? "Copied" : "Copy" }}
      </BaseButton>
    </PanelHeader>

    <pre v-if="commandLine" class="preview__code"><code>{{ commandLine }}</code></pre>
    <AlertBar
      v-else-if="showIssues"
      tone="error"
      :title="`${issueCount} ${issueCount === 1 ? 'issue' : 'issues'} to fix before running`"
    >
      <ul v-if="validation.errors.length" class="preview__list">
        <li v-for="e in validation.errors" :key="e">{{ e }}</li>
      </ul>
      <span v-else>Check the highlighted fields above.</span>
    </AlertBar>
    <AlertBar v-else tone="info">Fill in the required fields to see the command line.</AlertBar>
  </section>
</template>

<style scoped>
.preview {
  display: flex;
  flex-direction: column;
  gap: 8px;
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

.preview__list {
  margin: 4px 0 0;
  padding-left: 18px;
}
</style>
