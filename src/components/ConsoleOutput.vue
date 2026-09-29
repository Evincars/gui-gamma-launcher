<script setup lang="ts">
import { nextTick, ref, watch } from "vue";

import BaseButton from "./common/BaseButton.vue";
import PanelHeader from "./common/PanelHeader.vue";
import { useRunner } from "../composables/useRunner";

const { consoleLines, running, clearConsole } = useRunner();

const scroller = ref<HTMLElement | null>(null);
const stickToBottom = ref(true);

function onScroll() {
  const el = scroller.value;
  if (!el) return;
  stickToBottom.value = el.scrollHeight - el.scrollTop - el.clientHeight < 40;
}

watch(
  () => consoleLines.value[consoleLines.value.length - 1]?.id,
  async () => {
    if (!stickToBottom.value) return;
    await nextTick();
    const el = scroller.value;
    if (el) el.scrollTop = el.scrollHeight;
  },
);
</script>

<template>
  <section class="console">
    <PanelHeader title="Output">
      <template #badge>
        <span v-if="running" class="console__live">live</span>
      </template>
      <BaseButton :disabled="!consoleLines.length" @click="clearConsole">Clear</BaseButton>
    </PanelHeader>

    <div ref="scroller" class="console__body" @scroll="onScroll">
      <p v-if="!consoleLines.length" class="console__empty">
        Output from the launcher appears here.
      </p>
      <div
        v-for="line in consoleLines"
        :key="line.id"
        :class="['console__line', `console__line--${line.stream}`]"
      >
        {{ line.text }}
      </div>
    </div>
  </section>
</template>

<style scoped>
.console {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.console__live {
  padding: 2px 6px;
  border-radius: 999px;
  background: var(--accent-soft);
  color: var(--accent);
  font-size: 9px;
  letter-spacing: 0.06em;
}

/* Own scroll area with a fixed height, so the page itself can scroll past it. */
.console__body {
  height: clamp(220px, 45vh, 560px);
  overflow-y: auto;
  padding: 12px 14px;
  border-radius: var(--radius-sm);
  background: var(--bg-inset);
  border: 1px solid var(--panel-border);
  font-family: var(--font-mono);
  font-size: 12px;
  line-height: 1.55;
}

.console__empty {
  margin: 0;
  color: var(--text-faint);
}

.console__line {
  white-space: pre-wrap;
  word-break: break-word;
}

.console__line--stdout {
  color: var(--stream-stdout);
}
.console__line--stderr {
  color: var(--stream-stderr);
}
.console__line--error {
  color: var(--danger-hover);
  font-weight: 600;
}
.console__line--meta {
  color: var(--stream-meta);
  font-style: italic;
}
</style>
