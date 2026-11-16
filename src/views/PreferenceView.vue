<script setup lang="ts">
/** Route-backed preference sections grouped into vertical categories. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { preferenceGroups } from '@/components/layout/navigation'
import { usePreferenceActions } from '@/composables/usePreferenceActions'
import PreferenceActionBar from '@/components/preference/PreferenceActionBar.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const actions = usePreferenceActions()
const selectedSection = computed(() => String(route.name ?? 'preference-general').replace(/^preference-/, ''))
/** Group that owns the routed section; drives the mobile tab strip. */
const activeGroupKey = computed(
  () =>
    preferenceGroups.find((group) => (group.items as readonly string[]).includes(selectedSection.value))?.key ??
    preferenceGroups[0].key,
)

function selectGroup(key: string): void {
  const group = groups.value.find((item) => item.key === key)
  const first = group?.items[0]
  if (!first) return
  if (first.name === selectedSection.value) return
  void router.push({ name: 'preference-' + first.name })
}

const groups = computed(() =>
  preferenceGroups.map((group) => ({
    key: group.key,
    label:
      group.key === 'account'
        ? t('preferences.group-account')
        : group.key === 'downloads'
          ? t('preferences.group-download')
          : group.key === 'appearance'
            ? t('preferences.group-appearance')
            : t('preferences.group-advanced'),
    hint:
      group.key === 'account'
        ? t('preferences.group-account-hint')
        : group.key === 'downloads'
          ? t('preferences.group-download-hint')
          : group.key === 'appearance'
            ? t('preferences.group-appearance-hint')
            : t('preferences.group-advanced-hint'),
    items: group.items.map((item) => ({
      name: item,
      label:
        item === 'lan'
          ? t('preferences.lan')
          : item === 'permissions'
            ? t('preferences.permissions')
            : item === 'downloader'
              ? t('preferences.downloader')
              : item === 'categories'
                ? t('preferences.categories')
                : item === 'general'
                  ? t('preferences.general')
                  : item === 'plugins'
                    ? t('preferences.plugin-management')
                    : item === 'checksum'
                      ? t('preferences.checksum-title')
                      : t('preferences.advanced'),
    })),
  })),
)

const activeGroup = computed(() => groups.value.find((group) => group.key === activeGroupKey.value) ?? null)
</script>

<template>
  <div class="preference-view">
    <nav class="preference-tabs" role="tablist" :aria-label="t('navigation.settings')">
      <button
        v-for="group in groups"
        :key="group.key"
        type="button"
        role="tab"
        class="preference-tab"
        :class="{ active: group.key === activeGroupKey }"
        :aria-selected="group.key === activeGroupKey"
        @click="selectGroup(group.key)"
      >
        {{ group.label }}
      </button>
    </nav>
    <div class="preference-card-flow">
      <RouterLink
        v-for="item in activeGroup?.items ?? []"
        :key="item.name"
        :to="{ name: 'preference-' + item.name }"
        class="preference-card"
        :class="{ active: selectedSection === item.name }"
      >
        <span class="preference-card-label">{{ item.label }}</span>
        <span class="preference-card-hint">{{ activeGroup?.hint }}</span>
      </RouterLink>
    </div>
    <div class="preference-layout">
      <nav class="preference-groups" :aria-label="t('navigation.settings')">
        <section v-for="group in groups" :key="group.key" class="preference-group">
          <h3>{{ group.label }}</h3>
          <p>{{ group.hint }}</p>
          <RouterLink
            v-for="item in group.items"
            :key="item.name"
            :to="{ name: 'preference-' + item.name }"
            class="preference-group-link"
            :class="{ active: selectedSection === item.name }"
          >
            {{ item.label }}
          </RouterLink>
        </section>
      </nav>
      <div class="panel-body">
        <router-view v-slot="{ Component, route: innerRoute }">
          <Transition name="fade" mode="out-in">
            <component :is="Component" :key="innerRoute.path" />
          </Transition>
        </router-view>
        <div v-if="actions?.isDirty" class="preference-actions-card">
          <PreferenceActionBar
            :is-dirty="actions.isDirty"
            :is-valid="actions.isValid"
            @save="actions.save()"
            @discard="actions.discard()"
          />
        </div>
      </div>
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
/* Mobile tab strip and card flow stay hidden until the small-screen query. */
.preference-tabs,
.preference-card-flow {
  display: none;
}
.preference-layout {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-columns: minmax(210px, 260px) minmax(0, 1fr);
  gap: 16px;
  padding: 12px var(--content-gutter) 0;
}
.preference-groups {
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-bottom: 16px;
}
.preference-group {
  padding: 12px;
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
.preference-group h3 {
  margin: 0 0 3px;
  font-size: var(--font-size-sm);
}
.preference-group p {
  margin: 0 0 8px;
  color: var(--m3-on-surface-variant);
  font-size: var(--font-size-xs);
  line-height: 1.45;
}
.preference-group-link {
  display: flex;
  align-items: center;
  gap: 8px;
  min-height: 44px;
  padding: 0 10px;
  border-radius: var(--radius-md);
  color: var(--m3-on-surface-variant);
}
.preference-group-link::after {
  margin-left: auto;
  content: '›';
  color: var(--m3-outline);
  font-size: 18px;
  line-height: 1;
}
.preference-group-link:hover {
  background: var(--m3-surface-container-high);
}
.preference-group-link.active {
  color: var(--m3-on-primary-container);
  background: var(--m3-primary-container);
  font-weight: 600;
}
.panel-body {
  display: flex;
  flex-direction: column;
  flex: 1;
  width: 100%;
  max-width: 860px;
  justify-self: center;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
}
.panel-body > :deep(.preference-form-wrapper) {
  flex: 1;
  min-height: 0;
}
.preference-actions-card {
  /* Bottom dock: the form scrolls above it, so save/discard stay reachable. */
  flex: none;
  margin: 0 var(--content-gutter) 10px;
  padding: 10px 12px calc(10px + env(safe-area-inset-bottom, 0px));
  background: var(--m3-surface-container-low);
  border: 1px solid var(--m3-outline-variant);
  border-radius: var(--radius-lg);
}
/* Phones: a horizontal tab strip replaces the vertical group list, and the
   sections of the active group flow as cards underneath. The layout collapses
   at the same width, so tablet portrait keeps its two columns instead of
   stacking a second navigation above the form. */
@media (max-width: 768px) {
  .preference-layout {
    grid-template-columns: minmax(0, 1fr);
    gap: 10px;
    padding-inline: 8px;
  }
  .preference-tabs {
    display: flex;
    flex: none;
    gap: 6px;
    padding: 8px 10px 0;
    overflow-x: auto;
    overscroll-behavior-x: contain;
    scrollbar-width: none;
  }
  .preference-tabs::-webkit-scrollbar {
    display: none;
  }
  .preference-tab {
    flex: 0 0 auto;
    min-height: 44px;
    padding: 0 14px;
    color: var(--m3-on-surface-variant);
    background: var(--m3-surface-container-low);
    border: 1px solid var(--m3-outline-variant);
    border-radius: var(--radius-full);
    font-size: var(--font-size-sm);
    font-weight: 600;
    white-space: nowrap;
  }
  .preference-tab.active {
    color: var(--m3-on-primary-container);
    background: var(--m3-primary-container);
    border-color: transparent;
  }
  .preference-card-flow {
    display: grid;
    flex: none;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 8px;
    padding: 8px 10px;
  }
  .preference-card {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-height: 44px;
    padding: 8px 10px;
    background: var(--m3-surface-container-low);
    border: 1px solid var(--m3-outline-variant);
    border-radius: var(--radius-lg);
  }
  .preference-card.active {
    color: var(--m3-on-primary-container);
    background: var(--m3-primary-container);
    border-color: transparent;
  }
  .preference-card-label {
    font-size: var(--font-size-sm);
    font-weight: 600;
  }
  .preference-card-hint {
    overflow: hidden;
    color: var(--m3-on-surface-variant);
    font-size: var(--font-size-xs);
    line-height: 1.4;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 1;
  }
  .preference-groups {
    display: none;
  }
  .preference-layout {
    padding-top: 0;
  }
}
</style>
