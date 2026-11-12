<script setup lang="ts">
/** Route-backed preference tabs retain the shared unsaved-change guard. */
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import { NTabs, NTab } from 'naive-ui'
import { preferenceDestinations } from '@/components/layout/navigation'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const selectedTab = computed(() => String(route.name ?? 'preference-general'))
const tabsRef = ref<{ $el: HTMLElement } | null>(null)

/** Keeps the active tab visible in the horizontally scrollable mobile tab strip. */
async function revealActiveTab(): Promise<void> {
  await nextTick()
  const scroller = tabsRef.value?.$el.querySelector<HTMLElement>('.v-x-scroll')
  const active = scroller?.querySelector<HTMLElement>('.n-tabs-tab--active')
  if (!scroller || !active) return
  if (scroller.scrollWidth <= scroller.clientWidth) return
  const target = active.offsetLeft - (scroller.clientWidth - active.offsetWidth) / 2
  const max = scroller.scrollWidth - scroller.clientWidth
  scroller.scrollTo({ left: Math.max(0, Math.min(max, target)), behavior: 'smooth' })
}

watch(selectedTab, () => void revealActiveTab())
onMounted(() => void revealActiveTab())
function navigate(name: string) {
  void router.push({ name })
}
</script>

<template>
  <div class="preference-view">
    <NTabs ref="tabsRef" class="preference-tabs" :value="selectedTab" type="line" @update:value="navigate">
      <NTab v-for="key in preferenceDestinations" :key="key" :name="'preference-' + key">
        {{ t('preferences.' + key) }}
      </NTab>
    </NTabs>
    <div class="panel-body">
      <router-view v-slot="{ Component, route: innerRoute }">
        <Transition name="fade" mode="out-in">
          <component :is="Component" :key="innerRoute.path" />
        </Transition>
      </router-view>
    </div>
  </div>
</template>

<style scoped>
.preference-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}
.preference-tabs {
  padding: 8px var(--content-gutter) 0;
  flex-shrink: 0;
}
.panel-body {
  flex: 1;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
}
@media (max-width: 768px) {
  .preference-tabs {
    padding-inline: 8px;
    overflow-x: auto;
    scrollbar-width: none;
  }
  .preference-tabs::-webkit-scrollbar {
    display: none;
    width: 0;
    height: 0;
  }
  .preference-tabs :deep(.n-tabs-nav-scroll-content) {
    min-width: max-content;
  }
}
</style>
