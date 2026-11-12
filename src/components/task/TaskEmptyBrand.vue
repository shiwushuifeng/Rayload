<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useBrandLogo } from '@/composables/useBrandLogo'
import EmptyIllustration from '@/components/common/EmptyIllustration.vue'

withDefaults(defineProps<{ show: boolean; showLogo?: boolean }>(), { showLogo: true })
const { t } = useI18n()
const { brandLogo } = useBrandLogo()
</script>

<template>
  <Transition name="empty-brand">
    <div v-if="show" class="empty-brand">
      <div class="empty-brand-art-wrap">
        <EmptyIllustration variant="tasks" class="empty-brand-art" />
        <img v-if="showLogo" class="empty-brand-mark" :src="brandLogo" alt="" draggable="false" />
      </div>
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
.empty-brand-art-wrap {
  position: relative;
  width: min(clamp(190px, 42cqw, 300px), 86cqw, 58cqh);
}
.empty-brand-art {
  width: 100%;
  filter: drop-shadow(0 18px 34px color-mix(in srgb, var(--m3-primary) 18%, transparent));
}
.empty-brand-mark {
  position: absolute;
  right: 2%;
  bottom: 6%;
  width: clamp(40px, 11cqw, 58px);
  aspect-ratio: 1;
  object-fit: contain;
  padding: 7px;
  background: color-mix(in srgb, var(--m3-surface-container-lowest) 86%, transparent);
  border: 1px solid color-mix(in srgb, var(--m3-primary) 18%, var(--m3-outline-variant));
  border-radius: 50%;
  box-shadow: 0 8px 20px color-mix(in srgb, var(--m3-primary) 18%, transparent);
}
.empty-brand-hint {
  width: min(320px, 82cqw);
  margin: 0;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-base);
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
