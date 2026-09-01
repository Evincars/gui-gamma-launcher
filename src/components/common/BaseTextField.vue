<script setup lang="ts">
withDefaults(
  defineProps<{
    modelValue: string;
    placeholder?: string;
    disabled?: boolean;
    monospace?: boolean;
  }>(),
  { placeholder: "", disabled: false, monospace: false },
);

const emit = defineEmits<{ (e: "update:modelValue", value: string): void }>();
</script>

<template>
  <input
    class="text-field"
    :class="{ 'text-field--mono': monospace }"
    type="text"
    :value="modelValue"
    :placeholder="placeholder"
    :disabled="disabled"
    spellcheck="false"
    autocomplete="off"
    @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
  />
</template>

<style scoped>
.text-field {
  width: 100%;
  padding: 8px 10px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--panel-border-strong);
  background: var(--bg-inset);
  color: var(--text);
  font-size: 13px;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.text-field--mono {
  font-family: var(--font-mono);
}

.text-field::placeholder {
  color: var(--text-faint);
}

.text-field:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.text-field:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
