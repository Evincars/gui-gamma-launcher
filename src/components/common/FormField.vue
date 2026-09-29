<script setup lang="ts">
import { useId } from "vue";

import AlertBar from "./AlertBar.vue";
import InfoTip from "./InfoTip.vue";

defineProps<{
  label: string;
  description?: string;
  required?: boolean;
  flag?: string;
  /** Validation message; also switches the field to its invalid style. */
  error?: string;
  /** Small note next to the label, e.g. "saved". */
  badge?: string;
  /** Longer explanation shown in a hover tooltip. */
  info?: string;
}>();

defineSlots<{ default(props: { id: string; invalid: boolean }): unknown }>();

const id = useId();
</script>

<template>
  <div :class="['field', { 'field--invalid': error }]">
    <div class="field__head">
      <span class="field__title">
        <label class="field__label" :for="id">
          {{ label }}
          <span v-if="required" class="field__req" title="Required">*</span>
          <span v-if="badge" class="field__badge">{{ badge }}</span>
        </label>
        <InfoTip v-if="info" :text="info" :label="`About ${label}`" />
      </span>
      <code v-if="flag" class="field__flag">{{ flag }}</code>
    </div>
    <p v-if="description" class="field__desc">{{ description }}</p>
    <slot :id="id" :invalid="!!error" />
    <AlertBar v-if="error" tone="error" dense>{{ error }}</AlertBar>
  </div>
</template>

<style scoped>
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding-left: 10px;
  border-left: 2px solid transparent;
  transition: border-color 0.15s;
}

.field--invalid {
  border-left-color: var(--danger);
}

.field__head {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
}

.field__title {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.field__label {
  display: inline-flex;
  align-items: baseline;
  gap: 6px;
  font-size: 13px;
  font-weight: 600;
  color: var(--text);
}

.field--invalid .field__label {
  color: var(--danger-hover);
}

.field__req {
  color: var(--accent);
  margin-left: -4px;
}

.field__badge {
  padding: 1px 6px;
  border-radius: 999px;
  background: var(--accent-soft);
  color: var(--accent);
  font-size: 10px;
  font-weight: 500;
}

.field__flag {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-faint);
  white-space: nowrap;
}

.field__desc {
  margin: 0;
  font-size: 12px;
  color: var(--text-muted);
}
</style>
