<script setup lang="ts">
/** @fileoverview Reusable completed-download category summary. */
import type { Component } from 'vue'
import { useI18n } from 'vue-i18n'
import { NCard, NIcon } from 'naive-ui'
import { bytesToSize } from '@shared/utils/format'

defineProps<{
  title: string
  hint: string
  rows: Array<{
    key: string
    label: string
    icon: Component
    value: number
    count: number
    share: number
  }>
}>()

const { t } = useI18n()
</script>

<template>
  <NCard class="history-category-card" :bordered="false">
    <div class="history-section-heading">
      <div>
        <h2>{{ title }}</h2>
        <p>{{ hint }}</p>
      </div>
    </div>
    <div class="history-category-grid">
      <div v-for="row in rows" :key="row.key" class="history-category">
        <div class="history-category__icon">
          <NIcon :size="19"><component :is="row.icon" /></NIcon>
        </div>
        <div class="history-category__copy">
          <strong>{{ row.label }}</strong>
          <span>{{ bytesToSize(row.value) }} · {{ t('statistics.category-tasks', { count: row.count }) }}</span>
        </div>
        <span class="history-category__bar">
          <span :style="{ width: `${row.share}%` }" />
        </span>
      </div>
    </div>
  </NCard>
</template>

<style scoped>
.history-category-card {
  overflow: hidden;
  background: var(--m3-surface-container);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.history-section-heading {
  display: flex;
  gap: 12px;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 10px;
}
.history-section-heading h2 {
  margin: 0 0 3px;
  font-size: 17px;
}
.history-section-heading p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
}
.history-category-grid {
  display: grid;
  gap: 8px;
}
.history-category {
  display: grid;
  grid-template-columns: 38px minmax(0, 1fr) 96px;
  gap: 10px;
  align-items: center;
  padding: 8px 10px;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-md, 12px);
}
.history-category__icon {
  display: grid;
  width: 38px;
  height: 38px;
  place-items: center;
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  border-radius: 11px;
}
.history-category__copy {
  display: grid;
  gap: 2px;
  min-width: 0;
}
.history-category__copy strong {
  overflow: hidden;
  font-size: var(--font-size-sm);
  text-overflow: ellipsis;
  white-space: nowrap;
}
.history-category__copy span {
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  font-variant-numeric: tabular-nums;
}
.history-category__bar {
  display: block;
  height: 6px;
  overflow: hidden;
  background: var(--m3-surface-container-highest);
  border-radius: var(--radius-full);
}
.history-category__bar span {
  display: block;
  height: 100%;
  background: linear-gradient(90deg, var(--m3-primary), var(--m3-tertiary));
  border-radius: inherit;
  transition: width 0.4s cubic-bezier(0.2, 0, 0, 1);
}
@media (max-width: 520px) {
  .history-category {
    grid-template-columns: 38px minmax(0, 1fr) 72px;
  }
}
</style>
