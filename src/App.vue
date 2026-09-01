<script setup lang="ts">
import { onMounted } from "vue";

import AppHeader from "./components/AppHeader.vue";
import CommandForm from "./components/CommandForm.vue";
import CommandPreview from "./components/CommandPreview.vue";
import CommandSidebar from "./components/CommandSidebar.vue";
import ConsoleOutput from "./components/ConsoleOutput.vue";
import RunPanel from "./components/RunPanel.vue";
import { useGammaLauncher } from "./composables/useGammaLauncher";

const { loading, schemaError, load } = useGammaLauncher();

onMounted(load);
</script>

<template>
  <div class="app">
    <AppHeader />

    <div v-if="schemaError" class="app__fatal">
      Could not load command schema: {{ schemaError }}
    </div>

    <div v-else-if="loading" class="app__loading">Loading commands…</div>

    <div v-else class="app__body">
      <CommandSidebar />
      <main class="app__main">
        <div class="app__config">
          <CommandForm />
          <CommandPreview />
          <RunPanel />
        </div>
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

.app__fatal,
.app__loading {
  margin: 20px;
  padding: 16px;
  border-radius: var(--radius-sm);
  font-size: 13px;
}

.app__fatal {
  background: rgba(240, 96, 60, 0.1);
  border: 1px solid var(--danger);
  color: var(--danger-hover);
}

.app__loading {
  color: var(--text-muted);
}

.app__body {
  flex: 1;
  display: flex;
  min-height: 0;
}

.app__main {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 20px;
  overflow: hidden;
}

.app__config {
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow-y: auto;
  padding-right: 4px;
}
</style>
