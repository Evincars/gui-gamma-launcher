<script setup lang="ts">
import BaseButton from "./common/BaseButton.vue";
import FormField from "./common/FormField.vue";
import OptionInput from "./OptionInput.vue";
import { useCommandForm } from "../composables/useCommandForm";
import { useRunner } from "../composables/useRunner";
import { isSharedKey } from "../composables/useSharedPaths";
import { humanizeCommand, humanizeKey } from "../utils/format";

const { selectedCommand, getValue, setValue, visibleError, reset } = useCommandForm();
const { running } = useRunner();
</script>

<template>
  <section v-if="selectedCommand" class="cmd-form">
    <div class="cmd-form__head">
      <div>
        <h2>{{ humanizeCommand(selectedCommand.name) }}</h2>
        <p class="cmd-form__desc">{{ selectedCommand.description }}</p>
      </div>
      <BaseButton :disabled="running" title="Saved Anomaly / GAMMA paths are kept" @click="reset">
        Reset options
      </BaseButton>
    </div>

    <div class="cmd-form__fields">
      <FormField
        v-for="opt in selectedCommand.options"
        :key="opt.key"
        v-slot="{ id, invalid }"
        :class="{ 'cmd-form__wide': opt.type !== 'boolean' }"
        :label="humanizeKey(opt.key)"
        :description="opt.description"
        :required="opt.required"
        :flag="opt.flag"
        :error="visibleError(opt.key)"
        :badge="isSharedKey(opt.key) ? 'saved · shared' : undefined"
      >
        <OptionInput
          :id="id"
          :option="opt"
          :model-value="getValue(opt.key)"
          :invalid="invalid"
          :disabled="running"
          @update:model-value="setValue(opt.key, $event)"
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

/* Paths / text take a full row; switches share rows to keep long forms short. */
.cmd-form__fields {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px 20px;
}

.cmd-form__wide {
  grid-column: 1 / -1;
}
</style>
