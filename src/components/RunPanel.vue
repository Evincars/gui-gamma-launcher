<script setup lang="ts">
import { computed } from "vue";

import BaseButton from "./common/BaseButton.vue";
import { useGammaLauncher } from "../composables/useGammaLauncher";

const { running, canRun, previewError, lastResult, run, cancel } = useGammaLauncher();

const status = computed(() => {
  if (running.value) return { text: "Running…", tone: "run" as const };
  if (previewError.value) return { text: "Fix required options", tone: "warn" as const };
  if (lastResult.value) {
    return lastResult.value.success
      ? { text: `Done (exit ${lastResult.value.code ?? 0})`, tone: "ok" as const }
      : { text: `Failed (exit ${lastResult.value.code ?? "?"})`, tone: "err" as const };
  }
  return { text: "Ready", tone: "idle" as const };
});
</script>

<template>
  <section class="run-panel">
    <div class="run-panel__status" :class="`run-panel__status--${status.tone}`">
      <span class="run-panel__dot" />
      {{ status.text }}
    </div>

    <div class="run-panel__actions">
      <BaseButton v-if="running" variant="danger" @click="cancel">Cancel</BaseButton>
      <BaseButton variant="primary" :disabled="!canRun" @click="run">
        Run command
      </BaseButton>
    </div>
  </section>
</template>

<style scoped>
.run-panel {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 12px 14px;
  background: var(--panel);
  border: 1px solid var(--panel-border);
  border-radius: var(--radius-sm);
}

.run-panel__status {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
}

.run-panel__dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-faint);
}

.run-panel__status--run .run-panel__dot {
  background: var(--warn);
  animation: pulse 1s ease-in-out infinite;
}
.run-panel__status--run {
  color: var(--warn);
}
.run-panel__status--ok .run-panel__dot {
  background: var(--ok);
}
.run-panel__status--ok {
  color: var(--ok);
}
.run-panel__status--err .run-panel__dot,
.run-panel__status--warn .run-panel__dot {
  background: var(--danger);
}
.run-panel__status--err,
.run-panel__status--warn {
  color: var(--danger-hover);
}

.run-panel__actions {
  display: flex;
  gap: 8px;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}
</style>
