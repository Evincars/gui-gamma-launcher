<script setup lang="ts">
defineProps<{
  title: string;
  subtitle?: string;
  description?: string;
  active?: boolean;
  disabled?: boolean;
}>();

defineEmits<{ (e: "select"): void }>();
</script>

<template>
  <button
    type="button"
    :class="['nav-item', { 'nav-item--active': active }]"
    :disabled="disabled"
    :aria-current="active ? 'page' : undefined"
    @click="$emit('select')"
  >
    <span class="nav-item__title">{{ title }}</span>
    <code v-if="subtitle" class="nav-item__subtitle">{{ subtitle }}</code>
    <span v-if="description" class="nav-item__desc">{{ description }}</span>
  </button>
</template>

<style scoped>
.nav-item {
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
  transition:
    background 0.12s,
    border-color 0.12s;
}

.nav-item:hover:not(:disabled) {
  background: var(--panel);
}

.nav-item:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.nav-item--active {
  background: var(--accent-soft);
  border-color: var(--accent);
}

.nav-item__title {
  font-size: 13px;
  font-weight: 600;
}

.nav-item__subtitle {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-faint);
}

.nav-item__desc {
  font-size: 11px;
  color: var(--text-muted);
  line-height: 1.35;
}
</style>
