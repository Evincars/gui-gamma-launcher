<script setup lang="ts">
import { onMounted } from "vue";

import AppHeader from "./components/AppHeader.vue";
import CommandForm from "./components/CommandForm.vue";
import CommandPreview from "./components/CommandPreview.vue";
import CommandSidebar from "./components/CommandSidebar.vue";
import ConsoleOutput from "./components/ConsoleOutput.vue";
import RequirementsPanel from "./components/RequirementsPanel.vue";
import RunPanel from "./components/RunPanel.vue";
import AlertBar from "./components/common/AlertBar.vue";
import { useRequirements } from "./composables/useRequirements";
import { useSchema } from "./composables/useSchema";

const { loading, schemaError, load } = useSchema();
const { check } = useRequirements();

onMounted(() => {
  void load();
  void check();
});
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
        <RequirementsPanel />
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
</style>
