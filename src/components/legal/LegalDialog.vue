<script setup lang="ts">
/** @fileoverview In-app user agreement and privacy policy text. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NModal } from 'naive-ui'
import { CloseOutline } from '@vicons/ionicons5'

const props = defineProps<{
  show: boolean
  kind: 'eula' | 'privacy'
}>()

const emit = defineEmits<{ 'update:show': [value: boolean] }>()

const { t } = useI18n()
const title = computed(() => t(props.kind === 'eula' ? 'legal.eula-title' : 'legal.privacy-title'))
const sections = computed(() =>
  props.kind === 'eula'
    ? [{ key: 'technical' }, { key: 'copyright' }, { key: 'lawful' }, { key: 'liability' }]
    : [{ key: 'privacy-local' }, { key: 'privacy-tracking' }, { key: 'privacy-lan' }],
)
</script>

<template>
  <NModal
    :show="show"
    :mask-closable="true"
    :z-index="16000"
    @update:show="(value: boolean) => emit('update:show', value)"
  >
    <section class="legal-dialog" role="dialog" aria-modal="true" :aria-label="title">
      <header class="legal-dialog__head">
        <h2>{{ title }}</h2>
        <button
          type="button"
          class="legal-dialog__close"
          :aria-label="t('legal.close')"
          @click="emit('update:show', false)"
        >
          <NIcon :size="18"><CloseOutline /></NIcon>
        </button>
      </header>
      <div class="legal-dialog__body">
        <article v-for="section in sections" :key="section.key" class="legal-dialog__section">
          <h3>{{ t(`legal.${section.key}-title`) }}</h3>
          <p>{{ t(`legal.${section.key}-body`) }}</p>
        </article>
      </div>
      <NButton block type="primary" @click="emit('update:show', false)">{{ t('legal.close') }}</NButton>
    </section>
  </NModal>
</template>

<style scoped>
.legal-dialog {
  display: flex;
  flex-direction: column;
  gap: 14px;
  width: min(560px, calc(100vw - 28px));
  max-height: calc(100dvh - 40px);
  padding: 18px;
  color: var(--m3-on-surface);
  background: var(--m3-surface-container-high);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-xl);
  box-shadow: 0 18px 60px var(--m3-shadow);
}

.legal-dialog__head {
  display: flex;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
}

.legal-dialog__head h2 {
  margin: 0;
  font-size: var(--font-size-lg);
}

.legal-dialog__close {
  display: grid;
  width: 44px;
  height: 44px;
  place-items: center;
  color: var(--m3-on-surface-variant);
  background: transparent;
  border: none;
  border-radius: var(--radius-full);
  cursor: pointer;
}

.legal-dialog__body {
  display: grid;
  gap: 14px;
  overflow-y: auto;
  overscroll-behavior: contain;
}

.legal-dialog__section h3 {
  margin: 0 0 4px;
  font-size: var(--font-size-sm);
}

.legal-dialog__section p {
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  line-height: 1.6;
}
</style>
