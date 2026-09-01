<script setup lang="ts">
import { useGammaLauncher } from "../composables/useGammaLauncher";
import { humanizeCommand } from "../utils/format";

const { schema, selectedName, running, selectCommand } = useGammaLauncher();
</script>

<template>
  <nav class="sidebar">
    <p class="sidebar__heading">Commands</p>
    <ul class="sidebar__list">
      <li v-for="cmd in schema?.commands ?? []" :key="cmd.name">
        <button
          :class="['sidebar__item', { 'sidebar__item--active': cmd.name === selectedName }]"
          :disabled="running"
          @click="selectCommand(cmd.name)"
        >
          <span class="sidebar__name">{{ humanizeCommand(cmd.name) }}</span>
          <code class="sidebar__cli">{{ cmd.name }}</code>
          <span class="sidebar__desc">{{ cmd.description }}</span>
        </button>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.sidebar {
  width: 260px;
  flex-shrink: 0;
  background: var(--bg-elevated);
  border-right: 1px solid var(--panel-border);
  overflow-y: auto;
  padding: 14px 10px;
}

.sidebar__heading {
  margin: 4px 8px 10px;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-faint);
}

.sidebar__list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.sidebar__item {
  width: 100%;
  text-align: left;
  display: grid;
  gap: 3px;
  padding: 9px 10px;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text);
  cursor: pointer;
  transition: background 0.12s, border-color 0.12s;
}

.sidebar__item:hover:not(:disabled) {
  background: var(--panel);
}

.sidebar__item:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.sidebar__item--active {
  background: var(--accent-soft);
  border-color: var(--accent);
}

.sidebar__name {
  font-size: 13px;
  font-weight: 600;
}

.sidebar__cli {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-faint);
}

.sidebar__desc {
  font-size: 11px;
  color: var(--text-muted);
  line-height: 1.35;
}
</style>
