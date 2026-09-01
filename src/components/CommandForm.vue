<script setup lang="ts">
import { computed } from "vue";

import BaseButton from "./common/BaseButton.vue";
import BaseTextField from "./common/BaseTextField.vue";
import BaseToggle from "./common/BaseToggle.vue";
import FormField from "./common/FormField.vue";
import PathField from "./common/PathField.vue";
import { useGammaLauncher } from "../composables/useGammaLauncher";
import { humanizeCommand, humanizeKey } from "../utils/format";

const { selectedCommand, currentValues, running, resetCommand } = useGammaLauncher();

const options = computed(() => selectedCommand.value?.options ?? []);

function set(key: string, value: string | boolean) {
  currentValues.value[key] = value;
}
</script>

<template>
  <section v-if="selectedCommand" class="cmd-form">
    <div class="cmd-form__head">
      <div>
        <h2>{{ humanizeCommand(selectedCommand.name) }}</h2>
        <p class="cmd-form__desc">{{ selectedCommand.description }}</p>
      </div>
      <BaseButton :disabled="running" @click="resetCommand">Reset</BaseButton>
    </div>

    <div class="cmd-form__fields">
      <FormField
        v-for="opt in options"
        :key="opt.key"
        :label="humanizeKey(opt.key)"
        :description="opt.description"
        :required="opt.required"
        :flag="opt.flag"
      >
        <PathField
          v-if="opt.type === 'path'"
          :model-value="(currentValues[opt.key] as string) ?? ''"
          :label="humanizeKey(opt.key)"
          :disabled="running"
          @update:model-value="set(opt.key, $event)"
        />
        <BaseToggle
          v-else-if="opt.type === 'boolean'"
          :model-value="!!currentValues[opt.key]"
          :disabled="running"
          @update:model-value="set(opt.key, $event)"
        />
        <BaseTextField
          v-else
          :model-value="(currentValues[opt.key] as string) ?? ''"
          :disabled="running"
          @update:model-value="set(opt.key, $event)"
        />
      </FormField>
    </div>
  </section>
</template>

<style scoped>
.cmd-form {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.cmd-form__head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.cmd-form h2 {
  font-size: 15px;
}

.cmd-form__desc {
  margin: 4px 0 0;
  font-size: 12px;
  color: var(--text-muted);
}

.cmd-form__fields {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
</style>
