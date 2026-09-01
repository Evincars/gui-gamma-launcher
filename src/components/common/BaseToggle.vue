<script setup lang="ts">
withDefaults(
  defineProps<{ modelValue: boolean; disabled?: boolean }>(),
  { disabled: false },
);

const emit = defineEmits<{ (e: "update:modelValue", value: boolean): void }>();
</script>

<template>
  <button
    type="button"
    role="switch"
    :aria-checked="modelValue"
    :class="['toggle', { 'toggle--on': modelValue }]"
    :disabled="disabled"
    @click="emit('update:modelValue', !modelValue)"
  >
    <span class="toggle__track"><span class="toggle__thumb" /></span>
    <span class="toggle__state">{{ modelValue ? "On" : "Off" }}</span>
  </button>
</template>

<style scoped>
.toggle {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  padding: 0;
  border: 0;
  background: none;
  color: var(--text-muted);
  font-size: 12px;
  cursor: pointer;
}

.toggle:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toggle__track {
  width: 38px;
  height: 22px;
  border-radius: 999px;
  background: var(--bg-inset);
  border: 1px solid var(--panel-border-strong);
  display: inline-flex;
  align-items: center;
  padding: 2px;
  transition: background 0.15s, border-color 0.15s;
}

.toggle__thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--text-faint);
  transition: transform 0.15s, background 0.15s;
}

.toggle--on .toggle__track {
  background: var(--accent-soft);
  border-color: var(--accent);
}
.toggle--on .toggle__thumb {
  background: var(--accent);
  transform: translateX(16px);
}
.toggle--on .toggle__state {
  color: var(--accent);
}
</style>
