<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";

import BaseButton from "./BaseButton.vue";
import BaseTextField from "./BaseTextField.vue";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    /** Forwarded to the text input so a `<label for>` can target it. */
    id?: string;
    disabled?: boolean;
    invalid?: boolean;
    /** Used in the folder picker title. */
    label?: string;
    placeholder?: string;
  }>(),
  {
    id: undefined,
    disabled: false,
    invalid: false,
    label: "directory",
    placeholder: "/path/to/directory",
  },
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
      :id="id"
      :model-value="modelValue"
      :disabled="disabled"
      :invalid="invalid"
      :placeholder="placeholder"
      monospace
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
  min-width: 0;
}
</style>
