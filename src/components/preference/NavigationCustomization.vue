<script setup lang="ts">
/** Android bottom-navigation destination switches backed by localStorage. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NDivider, NIcon, NSwitch } from 'naive-ui'
import { InformationCircleOutline, TrashOutline } from '@vicons/ionicons5'
import { useTaskDestinations } from '@/components/layout/navigation'
import { type NavigationItemKey, useNavigationVisibility } from '@/composables/useNavigationVisibility'

const { t } = useI18n()
const taskDestinations = useTaskDestinations()
const { visibility, setItemVisible } = useNavigationVisibility()

const items = computed(() => [
  ...taskDestinations.value,
  { key: 'trash' as const, label: t('navigation.trash'), icon: TrashOutline },
  { key: 'about' as const, label: t('navigation.about'), icon: InformationCircleOutline },
])

function updateVisibility(key: NavigationItemKey, visible: boolean): void {
  setItemVisible(key, visible)
}
</script>

<template>
  <NDivider title-placement="left">{{ t('preferences.navigation-customization') }}</NDivider>
  <p class="pref-section-note navigation-customization-hint">
    {{ t('preferences.navigation-customization-hint') }}
  </p>
  <div class="navigation-customization-grid" role="group" :aria-label="t('preferences.navigation-customization')">
    <label v-for="item in items" :key="item.key" class="navigation-customization-item">
      <span class="navigation-customization-label">
        <NIcon :size="18" aria-hidden="true"><component :is="item.icon" /></NIcon>
        <span>{{ item.label }}</span>
      </span>
      <NSwitch
        :value="visibility[item.key]"
        :aria-label="item.label"
        @update:value="(visible: boolean) => updateVisibility(item.key, visible)"
      />
    </label>
  </div>
</template>

<style scoped>
.navigation-customization-hint {
  margin: 0 0 10px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-sm);
  line-height: 1.5;
}

.navigation-customization-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 8px;
  max-width: 720px;
  padding-left: 50px;
}

.navigation-customization-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-height: 48px;
  padding: 8px 12px;
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
  background: var(--m3-surface-container-low);
  cursor: pointer;
}

.navigation-customization-label {
  display: flex;
  align-items: center;
  gap: 9px;
  min-width: 0;
  color: var(--m3-on-surface);
  font-size: var(--font-size-sm);
}

@media (max-width: 768px) {
  .navigation-customization-grid {
    grid-template-columns: minmax(0, 1fr);
    padding-left: 0;
  }

  .navigation-customization-item {
    min-height: 52px;
  }
}
</style>
