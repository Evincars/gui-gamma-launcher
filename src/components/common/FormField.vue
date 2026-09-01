<script setup lang="ts">
defineProps<{
  label: string;
  description?: string;
  required?: boolean;
  flag?: string;
}>();
</script>

<template>
  <div class="field">
    <div class="field__head">
      <span class="field__label">
        {{ label }}
        <span v-if="required" class="field__req" title="Required">*</span>
      </span>
      <code v-if="flag" class="field__flag">{{ flag }}</code>
    </div>
    <p v-if="description" class="field__desc">{{ description }}</p>
    <div class="field__control">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field__head {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
}

.field__label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text);
}

.field__req {
  color: var(--accent);
  margin-left: 2px;
}

.field__flag {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-faint);
  white-space: nowrap;
}

.field__desc {
  margin: 0;
  font-size: 12px;
  color: var(--text-muted);
}
</style>
