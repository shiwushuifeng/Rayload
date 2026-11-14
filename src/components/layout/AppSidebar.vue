<script setup lang="ts">
/** Persistent task navigation and application destinations. */
import { computed } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import MTooltip from '@/components/common/MTooltip.vue'
import { NIcon } from 'naive-ui'
import {
  InformationCircleOutline,
  ExtensionPuzzleOutline,
  SettingsOutline,
  StatsChartOutline,
  SunnyOutline,
  TrashOutline,
  WifiOutline,
} from '@vicons/ionicons5'
import { useTaskStore } from '@/stores/task'
import { usePreferenceStore } from '@/stores/preference'
import { useAppStore } from '@/stores/app'
import { useBuildInfoStore } from '@/stores/build'
import SidebarCount from './SidebarCount.vue'
import { useTaskDestinations } from './navigation'
import { triggerHaptic } from '@/composables/useHaptics'
import { usePlatform } from '@/composables/usePlatform'
import { useNavigationVisibility } from '@/composables/useNavigationVisibility'
import { useAppMessage } from '@/composables/useAppMessage'

const props = defineProps<{ compact?: boolean; trashOpen?: boolean }>()

const emit = defineEmits<{ 'show-about': []; 'show-trash': [] }>()
const { t } = useI18n()
const route = useRoute()
const taskDestinations = useTaskDestinations()
const tasks = useTaskStore()
const preferences = usePreferenceStore()
const appStore = useAppStore()
const buildInfo = useBuildInfoStore()
const appMessage = useAppMessage()
const { isAndroid } = usePlatform()
const { isItemVisible } = useNavigationVisibility()
const isSettings = computed(() => route.matched.some((record) => record.name === 'preference'))
const isStatistics = computed(() => route.name === 'statistics')
const visibleTaskDestinations = computed(() =>
  taskDestinations.value.filter((item) => !props.compact || isItemVisible(item.key)),
)
const trashVisible = computed(() => !props.compact || isItemVisible('trash'))
const aboutVisible = computed(() => !props.compact || isItemVisible('about'))
const keepAwakeVisible = computed(() => !props.compact || isItemVisible('keep-awake'))
const statisticsVisible = computed(() => !props.compact || isItemVisible('statistics'))
const isLanDevices = computed(() => route.name === 'lan-devices')
const isPlugins = computed(() => route.name === 'plugins')
const lanVisible = computed(() => buildInfo.info.lan && (!props.compact || isItemVisible('lan-devices')))
const pluginsVisible = computed(() => buildInfo.info.pluginEngine && (!props.compact || isItemVisible('plugins')))

function showAbout(): void {
  triggerHaptic()
  emit('show-about')
}

function showTrash(): void {
  triggerHaptic()
  emit('show-trash')
}

function enterKeepAwake(): void {
  triggerHaptic()
  appStore.showKeepAwake()
  appMessage.success(t('app.keep-awake-toast'))
}
</script>

<template>
  <nav
    class="sidebar"
    :class="{ compact: props.compact, counts: preferences.config.sidebarTaskCounts }"
    :aria-label="t('app.task-list')"
  >
    <div class="sidebar-scopes">
      <MTooltip v-for="item in visibleTaskDestinations" :key="item.key" placement="right" :disabled="props.compact">
        <template #trigger>
          <RouterLink
            :to="{ name: 'task', params: { status: item.key } }"
            class="sidebar-item"
            active-class="active"
            :aria-label="
              preferences.config.sidebarTaskCounts ? `${item.label} ${tasks.taskCounts[item.key]}` : item.label
            "
            @click="triggerHaptic"
          >
            <NIcon :size="18" aria-hidden="true"><component :is="item.icon" /></NIcon>
            <span class="sidebar-label">{{ item.label }}</span>
            <Transition name="sidebar-count">
              <SidebarCount v-if="preferences.config.sidebarTaskCounts" :value="tasks.taskCounts[item.key]" />
            </Transition>
          </RouterLink>
        </template>
        {{ item.label }}
      </MTooltip>
    </div>
    <div class="sidebar-bottom">
      <MTooltip v-if="isAndroid && keepAwakeVisible" placement="right" :disabled="props.compact">
        <template #trigger>
          <button
            type="button"
            class="sidebar-item keep-awake-item"
            :aria-label="t('navigation.keep-awake')"
            @click="enterKeepAwake"
          >
            <NIcon :size="18" aria-hidden="true"><SunnyOutline /></NIcon>
            <span class="sidebar-label">{{ t('navigation.keep-awake') }}</span>
          </button>
        </template>
        {{ t('navigation.keep-awake') }}
      </MTooltip>
      <MTooltip v-if="lanVisible" placement="right" :disabled="props.compact">
        <template #trigger>
          <RouterLink
            :to="{ name: 'lan-devices' }"
            class="sidebar-item"
            :class="{ active: isLanDevices }"
            :aria-label="t('navigation.lan-devices')"
            :aria-current="isLanDevices ? 'page' : undefined"
            @click="triggerHaptic"
          >
            <NIcon :size="18" aria-hidden="true"><WifiOutline /></NIcon>
            <span class="sidebar-label">{{ t('navigation.lan-devices') }}</span>
          </RouterLink>
        </template>
        {{ t('navigation.lan-devices') }}
      </MTooltip>
      <MTooltip v-if="pluginsVisible" placement="right" :disabled="props.compact">
        <template #trigger>
          <RouterLink
            :to="{ name: 'plugins' }"
            class="sidebar-item"
            :class="{ active: isPlugins }"
            :aria-label="t('navigation.plugins')"
            :aria-current="isPlugins ? 'page' : undefined"
            @click="triggerHaptic"
          >
            <NIcon :size="18" aria-hidden="true"><ExtensionPuzzleOutline /></NIcon>
            <span class="sidebar-label">{{ t('navigation.plugins') }}</span>
          </RouterLink>
        </template>
        {{ t('navigation.plugins') }}
      </MTooltip>
      <MTooltip v-if="statisticsVisible" placement="right" :disabled="props.compact">
        <template #trigger>
          <RouterLink
            :to="{ name: 'statistics' }"
            class="sidebar-item"
            :class="{ active: isStatistics }"
            :aria-label="t('navigation.statistics')"
            :aria-current="isStatistics ? 'page' : undefined"
            @click="triggerHaptic"
          >
            <NIcon :size="18" aria-hidden="true"><StatsChartOutline /></NIcon>
            <span class="sidebar-label">{{ t('navigation.statistics') }}</span>
          </RouterLink>
        </template>
        {{ t('navigation.statistics') }}
      </MTooltip>
      <MTooltip placement="right" :disabled="props.compact">
        <template #trigger>
          <RouterLink
            :to="{ name: 'preference-general' }"
            class="sidebar-item"
            :class="{ active: isSettings }"
            :aria-label="t('navigation.settings')"
            :aria-current="isSettings ? 'page' : undefined"
            @click="triggerHaptic"
          >
            <NIcon :size="18" aria-hidden="true"><SettingsOutline /></NIcon>
            <span class="sidebar-label">{{ t('navigation.settings') }}</span>
          </RouterLink>
        </template>
        {{ t('navigation.settings') }}
      </MTooltip>
      <MTooltip v-if="isAndroid && trashVisible" placement="right" :disabled="props.compact || props.trashOpen">
        <template #trigger>
          <button
            type="button"
            class="sidebar-item"
            :class="{ active: props.trashOpen }"
            :aria-label="t('navigation.trash')"
            :aria-expanded="!!props.trashOpen"
            aria-haspopup="dialog"
            @click="showTrash"
          >
            <NIcon :size="18" aria-hidden="true"><TrashOutline /></NIcon>
            <span class="sidebar-label">{{ t('navigation.trash') }}</span>
          </button>
        </template>
        {{ t('navigation.trash') }}
      </MTooltip>
      <MTooltip v-if="aboutVisible" placement="right" :disabled="props.compact">
        <template #trigger>
          <button type="button" class="sidebar-item" :aria-label="t('navigation.about')" @click="showAbout">
            <NIcon :size="18" aria-hidden="true"><InformationCircleOutline /></NIcon>
            <span class="sidebar-label">{{ t('navigation.about') }}</span>
          </button>
        </template>
        {{ t('navigation.about') }}
      </MTooltip>
    </div>
  </nav>
</template>

<style scoped>
.sidebar {
  height: 100%;
  min-height: 0;
  display: flex;
  flex-direction: column;
  padding: 8px;
  gap: 16px;
  background: var(--sidebar-bg);
}
.sidebar-scopes {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
}
.sidebar-bottom {
  flex-shrink: 0;
}
.sidebar-item {
  display: flex;
  align-items: center;
  gap: 10px;
  min-height: 44px;
  position: relative;
  overflow: hidden;
  width: 100%;
  padding: 8px 10px;
  margin-bottom: 4px;
  border-radius: var(--radius-md);
  color: var(--m3-on-surface-variant);
  text-align: left;
  transition:
    padding var(--navigation-duration) var(--navigation-easing),
    gap var(--navigation-duration) var(--navigation-easing),
    background-color 180ms ease,
    color 180ms ease;
}
.sidebar-item :deep(.n-icon) {
  flex-shrink: 0;
}
.sidebar-label {
  flex: 1;
  min-width: 0;
  text-align: center;
  white-space: nowrap;
  overflow: clip visible;
  text-overflow: ellipsis;
  transition: opacity var(--navigation-duration) var(--navigation-easing);
}
.sidebar-item:hover {
  background: var(--sidebar-hover-bg);
}
.sidebar-item.active {
  background: var(--sidebar-active-bg);
  color: var(--m3-on-surface);
}
.keep-awake-item {
  color: color-mix(in srgb, var(--m3-primary) 78%, var(--m3-on-surface-variant));
}
.keep-awake-item:hover {
  color: var(--m3-primary);
}
.sidebar-item:focus-visible {
  outline: 2px solid var(--m3-primary);
  outline-offset: -2px;
}
.sidebar-count-enter-active,
.sidebar-count-leave-active {
  transition:
    opacity 180ms ease,
    transform 180ms ease;
}
.sidebar-count-enter-from,
.sidebar-count-leave-to {
  opacity: 0;
  scale: 0.92;
}
.compact .sidebar-item {
  padding-inline: calc((var(--sidebar-width) - 34px) / 2);
  gap: 0;
}
.compact .sidebar-label {
  opacity: 0;
}
.sidebar-scopes .sidebar-item {
  justify-content: center;
  gap: 8px;
}
.sidebar-scopes .sidebar-label {
  flex: 0 1 auto;
}
.sidebar :deep(.sidebar-count) {
  position: absolute;
  top: 50%;
  right: 10px;
  transform: translateY(-50%);
  transition:
    scale var(--navigation-duration) var(--navigation-easing),
    top var(--navigation-duration) var(--navigation-easing),
    right var(--navigation-duration) var(--navigation-easing),
    transform var(--navigation-duration) var(--navigation-easing),
    opacity var(--navigation-duration) var(--navigation-easing),
    font-size var(--navigation-duration) var(--navigation-easing);
}
.compact :deep(.sidebar-count) {
  transform: translateY(0);
  top: 1px;
  right: 1px;
  min-width: 14px;
  max-width: 30px;
  overflow: hidden;
  padding: 0 3px;
  font-size: 9px;
  line-height: 13px;
}
.sidebar.compact {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(44px, 1fr));
  align-items: stretch;
  gap: 2px;
  height: auto;
  min-height: 56px;
  overflow-x: auto;
  padding: 6px 8px calc(6px + env(safe-area-inset-bottom, 0px));
  border-top: 1px solid var(--m3-outline-variant);
  background: var(--m3-surface-container);
}
.sidebar.compact .sidebar-scopes,
.sidebar.compact .sidebar-bottom {
  display: contents;
}
.sidebar.compact .sidebar-item {
  flex-direction: column;
  justify-content: center;
  gap: 2px;
  width: auto;
  min-width: 44px;
  min-height: 48px;
  margin: 0;
  padding: 6px 4px;
  border-radius: var(--radius-lg);
  text-align: center;
}
.sidebar.compact .sidebar-label {
  display: none;
}
.sidebar.compact :deep(.sidebar-count) {
  top: 3px;
  right: 3px;
  transform: none;
}
@media (hover: none), (pointer: coarse) {
  .sidebar-item:hover {
    background: transparent;
  }
  .sidebar-item.active:hover {
    background: var(--sidebar-active-bg);
  }
}
@media (prefers-reduced-motion: reduce) {
  .sidebar-item,
  .sidebar :deep(.sidebar-count),
  .sidebar-label {
    transition-duration: 1ms;
  }
}
</style>
