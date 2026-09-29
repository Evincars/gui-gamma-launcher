<script setup lang="ts">
import { useId } from "vue";

import BaseIcon from "./BaseIcon.vue";

defineProps<{ text: string; label?: string }>();

const id = useId();
</script>

<template>
  <span class="info-tip">
    <button
      type="button"
      class="info-tip__trigger"
      :aria-label="label ?? 'More information'"
      :aria-describedby="id"
    >
      <BaseIcon name="info" />
    </button>
    <span :id="id" role="tooltip" class="info-tip__bubble">{{ text }}</span>
  </span>
</template>

<style scoped>
.info-tip {
  position: relative;
  display: inline-flex;
}

.info-tip__trigger {
  display: inline-flex;
  padding: 0;
  border: 0;
  background: none;
  color: var(--text-faint);
  font-size: 14px;
  cursor: help;
}

.info-tip__trigger:hover,
.info-tip__trigger:focus-visible {
  color: var(--accent);
  outline: none;
}

.info-tip__bubble {
  position: absolute;
  top: calc(100% + 8px);
  left: -10px;
  z-index: 20;
  width: max-content;
  max-width: min(420px, 70vw);
  padding: 10px 12px;
  border: 1px solid var(--panel-border-strong);
  border-radius: var(--radius-sm);
  background: var(--bg-elevated);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.45);
  color: var(--text);
  font-size: 12px;
  font-weight: 400;
  line-height: 1.5;
  white-space: pre-line;
  visibility: hidden;
  opacity: 0;
  transition: opacity 0.12s;
  pointer-events: none;
}

.info-tip:hover .info-tip__bubble,
.info-tip__trigger:focus-visible + .info-tip__bubble {
  visibility: visible;
  opacity: 1;
}
</style>
