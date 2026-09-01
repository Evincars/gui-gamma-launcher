<script setup lang="ts">
withDefaults(
  defineProps<{
    variant?: "primary" | "ghost" | "danger";
    disabled?: boolean;
    type?: "button" | "submit";
  }>(),
  { variant: "ghost", disabled: false, type: "button" },
);

defineEmits<{ (e: "click", ev: MouseEvent): void }>();
</script>

<template>
  <button
    :type="type"
    :class="['btn', `btn--${variant}`]"
    :disabled="disabled"
    @click="$emit('click', $event)"
  >
    <slot />
  </button>
</template>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--panel-border-strong);
  background: var(--bg-elevated);
  color: var(--text);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition:
    background 0.15s,
    border-color 0.15s,
    opacity 0.15s;
}

.btn:hover:not(:disabled) {
  border-color: var(--text-faint);
}

.btn:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.btn--primary {
  background: var(--accent);
  border-color: var(--accent);
  color: var(--accent-contrast);
  font-weight: 600;
}
.btn--primary:hover:not(:disabled) {
  background: var(--accent-hover);
  border-color: var(--accent-hover);
}

.btn--danger {
  background: transparent;
  border-color: var(--danger);
  color: var(--danger);
}
.btn--danger:hover:not(:disabled) {
  background: rgba(240, 96, 60, 0.12);
  border-color: var(--danger-hover);
  color: var(--danger-hover);
}
</style>
