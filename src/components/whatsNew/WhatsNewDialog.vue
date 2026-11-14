<script setup lang="ts">
/** @fileoverview Dismissible release notes shown once per installed version. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NModal, NTag } from 'naive-ui'
import { markWhatsNewSeen, whatsNewFor } from '@shared/whatsNew'

const props = defineProps<{
  show: boolean
  version: string
}>()

const emit = defineEmits<{ 'update:show': [value: boolean] }>()

const { t } = useI18n()
const entry = computed(() => whatsNewFor(props.version))

function close(): void {
  markWhatsNewSeen(props.version)
  emit('update:show', false)
}
</script>

<template>
  <NModal
    :show="show && entry !== null"
    :mask-closable="true"
    :z-index="1500"
    @update:show="(value: boolean) => (value ? emit('update:show', true) : close())"
  >
    <section v-if="entry" class="whats-new" role="dialog" aria-modal="true" :aria-label="t('whats-new.title')">
      <header class="whats-new__head">
        <h2>{{ t('whats-new.title') }}</h2>
        <NTag size="small" :bordered="false" type="success">v{{ entry.version }}</NTag>
      </header>
      <ul class="whats-new__list">
        <li v-for="key in entry.highlights" :key="key">{{ t(key) }}</li>
      </ul>
      <NButton block type="primary" @click="close">{{ t('whats-new.close') }}</NButton>
    </section>
  </NModal>
</template>

<style scoped>
.whats-new {
  display: flex;
  flex-direction: column;
  gap: 14px;
  width: min(480px, calc(100vw - 28px));
  padding: 18px;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}

.whats-new__head {
  display: flex;
  gap: 10px;
  align-items: center;
  justify-content: space-between;
}

.whats-new__head h2 {
  margin: 0;
  font-size: var(--font-size-lg);
}

.whats-new__list {
  display: grid;
  gap: 10px;
  padding-left: 18px;
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  line-height: 1.6;
}
</style>
