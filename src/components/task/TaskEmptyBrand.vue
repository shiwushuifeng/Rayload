<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useBrandLogo } from '@/composables/useBrandLogo'

withDefaults(defineProps<{ show: boolean; showLogo?: boolean }>(), { showLogo: true })
const { t } = useI18n()
const { brandLogo } = useBrandLogo()
</script>

<template>
  <Transition name="empty-brand">
    <div v-if="show" class="empty-brand">
      <img v-if="showLogo" class="empty-brand-mark" :src="brandLogo" alt="" draggable="false" />
      <p class="empty-brand-hint">{{ t('task.empty-hint') }}</p>
    </div>
  </Transition>
</template>

<style scoped>
.empty-brand {
  --empty-brand-enter-duration: 240ms;
  --empty-brand-leave-duration: 160ms;
  --empty-brand-easing: cubic-bezier(0.2, 0, 0, 1);
  position: absolute;
  inset: 16px var(--content-gutter);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 18px;
  container-type: size;
  pointer-events: none;
  user-select: none;
}
.empty-brand-mark {
  width: min(clamp(180px, 36cqw, 280px), 80cqw, 80cqh);
  aspect-ratio: 1;
  object-fit: contain;
  opacity: 0.64;
  filter: drop-shadow(0 16px 30px color-mix(in srgb, var(--m3-primary) 24%, transparent));
}
.empty-brand-hint {
  width: min(320px, 82cqw);
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  line-height: 1.6;
  text-align: center;
  text-wrap: balance;
}
.empty-brand-enter-active {
  transition: opacity var(--empty-brand-enter-duration) var(--empty-brand-easing);
}
.empty-brand-leave-active {
  transition: opacity var(--empty-brand-leave-duration) var(--empty-brand-easing);
}
.empty-brand-enter-from,
.empty-brand-leave-to {
  opacity: 0;
}
@media (prefers-reduced-motion: reduce) {
  .empty-brand-enter-active,
  .empty-brand-leave-active {
    transition-duration: 1ms;
  }
}
</style>
