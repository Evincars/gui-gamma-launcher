<script setup lang="ts">
import BaseIcon from "./BaseIcon.vue";

withDefaults(
  defineProps<{
    tone?: "error" | "warning" | "info";
    title?: string;
    /** Compact inline variant, e.g. under a form field. */
    dense?: boolean;
  }>(),
  { tone: "error", title: undefined, dense: false },
);
</script>

<template>
  <div
    :class="['alert', `alert--${tone}`, { 'alert--dense': dense }]"
    :role="dense ? undefined : tone === 'error' ? 'alert' : 'status'"
  >
    <BaseIcon class="alert__icon" :name="tone" />
    <div class="alert__body">
      <strong v-if="title" class="alert__title">{{ title }}</strong>
      <div class="alert__content"><slot /></div>
    </div>
    <div v-if="$slots.actions" class="alert__actions"><slot name="actions" /></div>
  </div>
</template>

<style scoped>
.alert {
  --tone: var(--danger);
  --tone-text: var(--danger-hover);
  --tone-soft: var(--danger-soft);

  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 12px;
  border: 1px solid var(--tone);
  border-left-width: 4px;
  border-radius: var(--radius-sm);
  background: var(--tone-soft);
  color: var(--tone-text);
  font-size: 12px;
  line-height: 1.45;
}

.alert--warning {
  --tone: var(--warn);
  --tone-text: var(--warn);
  --tone-soft: var(--warn-soft);
}

.alert--info {
  --tone: var(--panel-border-strong);
  --tone-text: var(--text-muted);
  --tone-soft: var(--panel);
}

.alert--dense {
  padding: 5px 10px;
  border-width: 0 0 0 3px;
  font-size: 12px;
}

.alert__icon {
  font-size: 14px;
  margin-top: 1px;
}

.alert__body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.alert__title {
  font-size: 13px;
}

.alert__content {
  overflow-wrap: anywhere;
}

.alert__actions {
  flex-shrink: 0;
  display: flex;
  gap: 8px;
}
</style>
