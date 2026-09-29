<script setup lang="ts">
import ExternalLink from "./common/ExternalLink.vue";
import { useRequirements } from "../composables/useRequirements";
import { useSchema } from "../composables/useSchema";

const AUTHOR_URL = "https://lasak.netlify.app/";
const UPSTREAM_URL = "https://github.com/Mord3rca/gamma-launcher";

const { schema } = useSchema();
const { version } = useRequirements();
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
      <div class="app-header__row">
        <code v-if="schema" class="app-header__binary">{{ schema.binary }}</code>
        <span v-if="version" class="app-header__version">{{ version }}</span>
      </div>
      <p class="app-header__credits">
        Created by
        <ExternalLink :href="AUTHOR_URL">Evincars</ExternalLink>
        · based on
        <ExternalLink :href="UPSTREAM_URL" title="The command-line launcher this GUI wraps">
          Mord3rca/gamma-launcher
        </ExternalLink>
      </p>
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
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
  font-size: 11px;
}

.app-header__row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.app-header__credits {
  margin: 0;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  color: var(--text-faint);
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
