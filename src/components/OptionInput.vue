<script setup lang="ts">
import { computed } from "vue";

import BaseTextField from "./common/BaseTextField.vue";
import BaseToggle from "./common/BaseToggle.vue";
import PathField from "./common/PathField.vue";
import ModOrganizerVersionInput from "./ModOrganizerVersionInput.vue";
import type { GammaOption, GammaOptionValue } from "../lib/gammaLauncher";
import { humanizeKey } from "../utils/format";

// Renders the control matching an option's type.
const props = defineProps<{
  option: GammaOption;
  modelValue: GammaOptionValue;
  id?: string;
  disabled?: boolean;
  invalid?: boolean;
}>();

const emit = defineEmits<{ (e: "update:modelValue", value: GammaOptionValue): void }>();

const text = computed(() => (typeof props.modelValue === "string" ? props.modelValue : ""));
</script>

<template>
  <PathField
    v-if="option.type === 'path'"
    :id="id"
    :model-value="text"
    :label="humanizeKey(option.key)"
    :placeholder="option.placeholder || undefined"
    :disabled="disabled"
    :invalid="invalid"
    @update:model-value="emit('update:modelValue', $event)"
  />
  <BaseToggle
    v-else-if="option.type === 'boolean'"
    :id="id"
    :model-value="modelValue === true"
    :disabled="disabled"
    @update:model-value="emit('update:modelValue', $event)"
  />
  <ModOrganizerVersionInput
    v-else-if="option.format === 'modorganizer-tag'"
    :id="id"
    :model-value="text"
    :placeholder="option.placeholder"
    :disabled="disabled"
    :invalid="invalid"
    @update:model-value="emit('update:modelValue', $event)"
  />
  <BaseTextField
    v-else
    :id="id"
    :model-value="text"
    :placeholder="option.placeholder"
    :disabled="disabled"
    :invalid="invalid"
    monospace
    @update:model-value="emit('update:modelValue', $event)"
  />
</template>
