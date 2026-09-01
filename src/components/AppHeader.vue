<script setup lang="ts">
import { useGammaLauncher } from "../composables/useGammaLauncher";

const { version, schema } = useGammaLauncher();
</script>

<template>
  <header class="app-header">
    <div class="app-header__title">
      <span class="app-header__mark">☢</span>
      <div>
        <h1>GAMMA Launcher</h1>
        <p class="app-header__sub">S.T.A.L.K.E.R.: G.A.M.M.A. installer &amp; tools</p>
      </div>
    </div>
    <div class="app-header__meta">
      <code v-if="schema" class="app-header__binary">{{ schema.binary }}</code>
      <span v-if="version" class="app-header__version">{{ version }}</span>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  background: var(--bg-elevated);
  border-bottom: 1px solid var(--panel-border);
}

.app-header__title {
  display: flex;
  align-items: center;
  gap: 12px;
}

.app-header__mark {
  font-size: 26px;
  color: var(--accent);
  filter: drop-shadow(0 0 8px var(--accent-soft));
}

.app-header h1 {
  font-size: 16px;
  letter-spacing: 0.02em;
}

.app-header__sub {
  margin: 2px 0 0;
  font-size: 12px;
  color: var(--text-muted);
}

.app-header__meta {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 11px;
}

.app-header__binary {
  font-family: var(--font-mono);
  color: var(--text-faint);
}

.app-header__version {
  padding: 3px 8px;
  border-radius: 999px;
  background: var(--accent-soft);
  color: var(--accent);
  font-weight: 600;
}
</style>
