<script setup lang="ts">
import { computed } from "vue";

import BaseButton from "./common/BaseButton.vue";
import { useCommandForm } from "../composables/useCommandForm";
import { useRunner } from "../composables/useRunner";

const { request, submitted, issueCount, submit, validate } = useCommandForm();
const { running, lastResult, run, cancel } = useRunner();

type Tone = "idle" | "run" | "ok" | "err";

const status = computed<{ text: string; tone: Tone }>(() => {
  if (running.value) return { text: "Running…", tone: "run" };
  if (submitted.value && issueCount.value) {
    const n = issueCount.value;
    return { text: `Fix ${n} ${n === 1 ? "issue" : "issues"}`, tone: "err" };
  }
  if (lastResult.value) {
    const { success, code } = lastResult.value;
    return success
      ? { text: `Done (exit ${code ?? 0})`, tone: "ok" }
      : { text: `Failed (exit ${code ?? "?"})`, tone: "err" };
  }
  return { text: "Ready", tone: "idle" };
});

async function onRun() {
  const req = request.value;
  const { commandLine } = await submit();
  if (!req || !commandLine) return;
  await run(req, commandLine);
  // The run may have created/changed directories other commands depend on.
  void validate();
}
</script>

<template>
  <section class="run-panel">
    <div :class="['run-panel__status', `run-panel__status--${status.tone}`]" role="status">
      <span class="run-panel__dot" />
      {{ status.text }}
    </div>

    <div class="run-panel__actions">
      <BaseButton v-if="running" variant="danger" @click="cancel">Cancel</BaseButton>
      <BaseButton variant="primary" :disabled="running || !request" @click="onRun">
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
  background: currentColor;
}

.run-panel__status--idle .run-panel__dot {
  background: var(--text-faint);
}
.run-panel__status--run {
  color: var(--warn);
}
.run-panel__status--run .run-panel__dot {
  animation: pulse 1s ease-in-out infinite;
}
.run-panel__status--ok {
  color: var(--ok);
}
.run-panel__status--err {
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
