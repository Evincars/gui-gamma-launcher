<script setup lang="ts">
import { computed, ref, watch } from "vue";

import BaseButton from "./common/BaseButton.vue";
import BaseIcon, { type IconName } from "./common/BaseIcon.vue";
import { useRequirements } from "../composables/useRequirements";
import type { RequirementStatus } from "../lib/gammaLauncher";

const { checks, checking, checkError, errorCount, warningCount, check } = useRequirements();

const ICONS: Record<RequirementStatus, IconName> = { ok: "success", warning: "warning", error: "error" };

const overall = computed<RequirementStatus>(() =>
  errorCount.value || checkError.value ? "error" : warningCount.value ? "warning" : "ok",
);

const summary = computed(() => {
  if (checking.value && !checks.value.length) return "Checking system requirements…";
  if (checkError.value) return "Could not check system requirements";
  const parts = [
    errorCount.value && `${errorCount.value} missing`,
    warningCount.value && `${warningCount.value} ${warningCount.value === 1 ? "warning" : "warnings"}`,
  ].filter(Boolean);
  return parts.length ? `System requirements: ${parts.join(", ")}` : "System requirements: all good";
});

// Expanded while something needs attention; the user can still toggle it.
const open = ref(false);
watch(overall, (s) => (open.value = s !== "ok"), { immediate: true });
</script>

<template>
  <details
    :class="['reqs', `reqs--${overall}`]"
    :open="open"
    @toggle="open = ($event.target as HTMLDetailsElement).open"
  >
    <summary class="reqs__summary">
      <BaseIcon :name="ICONS[overall]" class="reqs__status" />
      <span class="reqs__title">{{ summary }}</span>
      <BaseButton :disabled="checking" @click.prevent="check">
        <BaseIcon name="refresh" />
        {{ checking ? "Checking…" : "Re-check" }}
      </BaseButton>
    </summary>

    <p v-if="checkError" class="reqs__error">{{ checkError }}</p>
    <p v-else-if="checks.length" class="reqs__intro">
      <template v-if="overall === 'ok'">
        Everything the installer needs is in place — you're good to go.
      </template>
      <template v-else>
        No worries — a few things on your system just need a quick setup. Follow the steps
        under each item marked in red (required) or yellow (recommended), then press
        <strong>Re-check</strong>. Once everything is green, the installation can run successfully.
      </template>
    </p>
    <ul class="reqs__list">
      <li v-for="c in checks" :key="c.id" :class="['reqs__item', `reqs__item--${c.status}`]">
        <BaseIcon :name="ICONS[c.status]" class="reqs__item-icon" />
        <div class="reqs__item-body">
          <span class="reqs__label">{{ c.label }}</span>
          <code class="reqs__detail">{{ c.detail }}</code>
          <p v-if="c.hint" class="reqs__hint">{{ c.hint }}</p>
        </div>
      </li>
    </ul>
  </details>
</template>

<style scoped>
.reqs {
  --tone: var(--ok);
  border: 1px solid var(--panel-border);
  border-left: 4px solid var(--tone);
  border-radius: var(--radius-sm);
  background: var(--panel);
}

.reqs--warning {
  --tone: var(--warn);
}
.reqs--error {
  --tone: var(--danger);
}

.reqs__summary {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  cursor: pointer;
  list-style: none;
  font-size: 13px;
  font-weight: 600;
}

.reqs__summary::-webkit-details-marker {
  display: none;
}

.reqs__status {
  color: var(--tone);
  font-size: 16px;
}

.reqs__title {
  flex: 1;
}

.reqs__error {
  margin: 0 12px 8px;
  color: var(--danger-hover);
  font-size: 12px;
}

.reqs__intro {
  margin: 0 12px 6px;
  color: var(--text-muted);
  font-size: 12px;
  line-height: 1.5;
}

.reqs__list {
  list-style: none;
  margin: 0;
  padding: 4px 12px 12px;
  display: grid;
  gap: 10px;
}

.reqs__item {
  display: flex;
  gap: 10px;
  font-size: 12px;
}

.reqs__item-icon {
  margin-top: 2px;
  font-size: 14px;
  color: var(--ok);
}
.reqs__item--warning .reqs__item-icon {
  color: var(--warn);
}
.reqs__item--error .reqs__item-icon {
  color: var(--danger);
}

.reqs__item-body {
  min-width: 0;
  display: grid;
  gap: 2px;
}

.reqs__label {
  font-weight: 600;
}

.reqs__detail {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-muted);
  overflow-wrap: anywhere;
}

.reqs__hint {
  margin: 4px 0 0;
  padding: 6px 10px;
  border-radius: var(--radius-sm);
  background: var(--bg-inset);
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 11px;
  white-space: pre-wrap;
}
</style>
