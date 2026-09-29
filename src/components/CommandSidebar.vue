<script setup lang="ts">
import { computed } from "vue";

import NavItem from "./common/NavItem.vue";
import { useCommandForm } from "../composables/useCommandForm";
import { useRunner } from "../composables/useRunner";
import { useSchema } from "../composables/useSchema";
import type { GammaCommandGroup } from "../lib/gammaLauncher";
import { humanizeCommand } from "../utils/format";

const { schema } = useSchema();
const { selectedName, select } = useCommandForm();
const { running } = useRunner();

const GROUPS: { id: GammaCommandGroup; label: string }[] = [
  { id: "main", label: "Install & verify" },
  { id: "tools", label: "Tools" },
];

const groups = computed(() =>
  GROUPS.map((g) => ({
    ...g,
    commands: schema.value?.commands.filter((c) => c.group === g.id) ?? [],
  })).filter((g) => g.commands.length),
);
</script>

<template>
  <nav class="sidebar" aria-label="Commands">
    <section v-for="group in groups" :key="group.id" class="sidebar__group">
      <p class="sidebar__heading">{{ group.label }}</p>
      <ul class="sidebar__list">
        <li v-for="cmd in group.commands" :key="cmd.name">
          <NavItem
            :title="humanizeCommand(cmd.name)"
            :subtitle="cmd.name"
            :description="cmd.description"
            :active="cmd.name === selectedName"
            :disabled="running"
            @select="select(cmd.name)"
          />
        </li>
      </ul>
    </section>
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

.sidebar__group + .sidebar__group {
  margin-top: 14px;
  padding-top: 14px;
  border-top: 1px solid var(--panel-border-strong);
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
</style>
