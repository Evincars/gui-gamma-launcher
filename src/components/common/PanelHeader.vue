<script setup lang="ts">
defineProps<{ title: string }>();
</script>

<template>
  <div class="panel-header">
    <span class="panel-header__title">
      {{ title }}
      <slot name="badge" />
    </span>
    <div v-if="$slots.default" class="panel-header__actions"><slot /></div>
  </div>
</template>

<style scoped>
.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-height: 34px;
}

.panel-header__title {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-faint);
}

.panel-header__actions {
  display: flex;
  gap: 8px;
}
</style>
