<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";

import BaseButton from "./BaseButton.vue";
import BaseTextField from "./BaseTextField.vue";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    disabled?: boolean;
    label?: string;
  }>(),
  { disabled: false, label: "directory" },
);

const emit = defineEmits<{ (e: "update:modelValue", value: string): void }>();

async function browse() {
  const picked = await open({
    directory: true,
    multiple: false,
    title: `Select ${props.label}`,
    defaultPath: props.modelValue || undefined,
  });
  if (typeof picked === "string") emit("update:modelValue", picked);
}
</script>

<template>
  <div class="path-field">
    <BaseTextField
      :model-value="modelValue"
      :disabled="disabled"
      monospace
      placeholder="/path/to/directory"
      @update:model-value="emit('update:modelValue', $event)"
    />
    <BaseButton :disabled="disabled" @click="browse">Browse…</BaseButton>
  </div>
</template>

<style scoped>
.path-field {
  display: flex;
  gap: 8px;
  align-items: stretch;
}
.path-field > :first-child {
  flex: 1;
}
</style>
