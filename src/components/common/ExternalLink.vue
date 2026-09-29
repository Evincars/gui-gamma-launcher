<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";

import BaseIcon from "./BaseIcon.vue";

// Opens in the system browser; a plain <a> would navigate the app's webview.
const props = defineProps<{ href: string }>();

async function open() {
  try {
    await openUrl(props.href);
  } catch (e) {
    console.error(`Could not open ${props.href}`, e);
  }
}
</script>

<template>
  <a class="ext-link" :href="href" rel="noopener noreferrer" @click.prevent="open">
    <slot />
    <BaseIcon name="external" />
  </a>
</template>

<style scoped>
.ext-link {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  color: var(--text-muted);
  text-decoration: none;
}

.ext-link:hover,
.ext-link:focus-visible {
  color: var(--accent);
  text-decoration: underline;
  outline: none;
}
</style>
