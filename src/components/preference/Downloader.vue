<script setup lang="ts">
/** @fileoverview Downloader settings: download, BitTorrent, ED2K and network sections. */
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { NTab, NTabs } from 'naive-ui'
import Downloads from './Downloads.vue'
import BitTorrent from './BitTorrent.vue'
import Ed2k from './Ed2k.vue'
import Network from './Network.vue'
import Advanced from './Advanced.vue'

const { t } = useI18n()
const route = useRoute()

type DownloaderSection = 'downloads' | 'bt' | 'ed2k' | 'network' | 'advanced'
const SECTIONS: readonly DownloaderSection[] = ['downloads', 'bt', 'ed2k', 'network', 'advanced']

const initial = String(route.query.section ?? '')
const section = ref<DownloaderSection>(
  SECTIONS.includes(initial as DownloaderSection) ? (initial as DownloaderSection) : 'downloads',
)
</script>

<template>
  <div class="downloader-preference">
    <NTabs
      v-model:value="section"
      class="downloader-tabs"
      type="segment"
      size="small"
      :tabs-padding="8"
      :aria-label="t('preferences.downloader')"
    >
      <NTab v-for="key in SECTIONS" :key="key" :name="key">{{ t(`preferences.${key}`) }}</NTab>
    </NTabs>
    <div class="downloader-body">
      <Downloads v-if="section === 'downloads'" />
      <BitTorrent v-else-if="section === 'bt'" />
      <Ed2k v-else-if="section === 'ed2k'" />
      <Network v-else-if="section === 'network'" />
      <Advanced v-else variant="engine" />
    </div>
  </div>
</template>

<style scoped>
.downloader-preference {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}

.downloader-tabs {
  padding: 8px var(--content-gutter) 0;
  flex-shrink: 0;
}

.downloader-body {
  flex: 1;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
}
</style>
