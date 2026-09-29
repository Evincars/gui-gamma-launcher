<script setup lang="ts">
import { onMounted } from "vue";

import AppHeader from "./components/AppHeader.vue";
import CommandForm from "./components/CommandForm.vue";
import CommandPreview from "./components/CommandPreview.vue";
import CommandSidebar from "./components/CommandSidebar.vue";
import ConsoleOutput from "./components/ConsoleOutput.vue";
import RunPanel from "./components/RunPanel.vue";
import AlertBar from "./components/common/AlertBar.vue";
import BaseButton from "./components/common/BaseButton.vue";
import { useSchema } from "./composables/useSchema";

const { loading, schemaError, binaryError, load, checkBinary } = useSchema();

onMounted(load);
</script>

<template>
  <div class="app">
    <AppHeader />

    <div v-if="schemaError" class="app__message">
      <AlertBar tone="error" title="Could not load command schema">{{ schemaError }}</AlertBar>
    </div>

    <div v-else-if="loading" class="app__message app__loading">Loading commands…</div>

    <div v-else class="app__body">
      <CommandSidebar />
      <main class="app__main">
        <AlertBar v-if="binaryError" tone="error" title="gamma-launcher cannot start">
          <pre class="app__pre">{{ binaryError }}</pre>
          <template #actions>
            <BaseButton @click="checkBinary">Re-check</BaseButton>
          </template>
        </AlertBar>
        <CommandForm />
        <CommandPreview />
        <RunPanel class="app__run" />
        <ConsoleOutput />
      </main>
    </div>
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.app__message {
  margin: 20px;
}

.app__loading {
  color: var(--text-muted);
  font-size: 13px;
}

.app__body {
  flex: 1;
  display: flex;
  min-height: 0;
}

/* The whole content column scrolls; the sidebar scrolls independently. */
.app__main {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 20px;
}

/* Keep Run / Cancel reachable while scrolling a long form. */
.app__run {
  position: sticky;
  bottom: 0;
  z-index: 1;
  box-shadow: 0 -8px 16px var(--bg);
}

.app__pre {
  margin: 4px 0 0;
  font-family: var(--font-mono);
  font-size: 11px;
  white-space: pre-wrap;
}
</style>
