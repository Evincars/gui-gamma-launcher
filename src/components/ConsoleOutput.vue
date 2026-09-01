<script setup lang="ts">
import { nextTick, ref, watch } from "vue";

import BaseButton from "./common/BaseButton.vue";
import { useGammaLauncher } from "../composables/useGammaLauncher";

const { consoleLines, running, clearConsole } = useGammaLauncher();

const scroller = ref<HTMLElement | null>(null);
const stickToBottom = ref(true);

function onScroll() {
  const el = scroller.value;
  if (!el) return;
  stickToBottom.value = el.scrollHeight - el.scrollTop - el.clientHeight < 40;
}

watch(
  () => consoleLines.value.length,
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
    <div class="console__head">
      <span class="console__title">
        Output
        <span v-if="running" class="console__live">live</span>
      </span>
      <BaseButton :disabled="!consoleLines.length" @click="clearConsole">Clear</BaseButton>
    </div>

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
  min-height: 0;
  flex: 1;
}

.console__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.console__title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-faint);
  display: inline-flex;
  align-items: center;
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

.console__body {
  flex: 1;
  min-height: 160px;
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
.console__line--meta {
  color: var(--stream-meta);
  font-style: italic;
}
</style>
