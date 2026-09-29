<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

import BaseTextField from "./common/BaseTextField.vue";
import { fetchModOrganizerReleases, type ModOrganizerRelease } from "../lib/github";

// ModOrganizer release tag input with suggestions from GitHub releases.
const props = defineProps<{
  modelValue: string;
  id?: string;
  placeholder?: string;
  disabled?: boolean;
  invalid?: boolean;
}>();

const emit = defineEmits<{ (e: "update:modelValue", value: string): void }>();

const releases = ref<ModOrganizerRelease[] | null>(null);
const loadError = ref(false);
const listId = `${props.id ?? "mo"}-releases`;

onMounted(async () => {
  try {
    releases.value = await fetchModOrganizerReleases();
  } catch {
    loadError.value = true;
  }
});

const latestStable = computed(() => releases.value?.find((r) => !r.prerelease)?.tag ?? null);

const unknownTag = computed(() => {
  const tag = props.modelValue.trim();
  return !!tag && !!releases.value && !releases.value.some((r) => r.tag === tag);
});
</script>

<template>
  <div class="mo-version">
    <BaseTextField
      :id="id"
      :model-value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      :invalid="invalid"
      :list="listId"
      monospace
      @update:model-value="emit('update:modelValue', $event)"
    />
    <datalist :id="listId">
      <option v-for="r in releases ?? []" :key="r.tag" :value="r.tag">
        {{ r.prerelease ? "pre-release" : "stable" }}
      </option>
    </datalist>

    <p class="mo-version__hint">
      <template v-if="latestStable">
        Latest stable release: <code>{{ latestStable }}</code>
        <button
          v-if="modelValue.trim() !== latestStable"
          type="button"
          class="mo-version__use"
          :disabled="disabled"
          @click="emit('update:modelValue', latestStable)"
        >
          use
        </button>
      </template>
      <template v-else-if="loadError">Could not load ModOrganizer releases from GitHub.</template>
      <template v-else>Loading ModOrganizer releases…</template>
    </p>
    <p v-if="unknownTag" class="mo-version__warn">
      No published release with Mod.Organizer-{{ modelValue.trim().replace(/^v/, "") }}.7z — the
      download will fail.
    </p>
  </div>
</template>

<style scoped>
.mo-version {
  display: grid;
  gap: 4px;
}

.mo-version__hint,
.mo-version__warn {
  margin: 0;
  font-size: 11px;
  color: var(--text-faint);
}

.mo-version__hint code {
  font-family: var(--font-mono);
  color: var(--text-muted);
}

.mo-version__warn {
  color: var(--warn);
}

.mo-version__use {
  margin-left: 4px;
  padding: 0 6px;
  border: 1px solid var(--panel-border-strong);
  border-radius: 999px;
  background: none;
  color: var(--accent);
  font-size: 10px;
  cursor: pointer;
}

.mo-version__use:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
