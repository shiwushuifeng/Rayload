<script setup lang="ts">
/** @fileoverview JSON text transfer for Rayload preference backups. */
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import { relaunch } from '@tauri-apps/plugin-process'
import { NAlert, NButton, NCard, NIcon, NInput, NModal, NSpin, useDialog } from 'naive-ui'
import { CopyOutline } from '@vicons/ionicons5'
import { usePreferenceStore } from '@/stores/preference'
import { useEngineStore } from '@/stores/engine'
import { useAppMessage } from '@/composables/useAppMessage'
import { buildSettingsBackup, parseSettingsBackup } from '@shared/utils/settingsBackup'
import { buildSystemConfigFromAppConfig } from '@shared/utils/systemConfig'
import { writeAppClipboardText } from '@shared/utils'
import { logger } from '@shared/logger'
import type { AppConfig } from '@shared/types'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ close: [] }>()

const { t } = useI18n()
const preferenceStore = usePreferenceStore()
const engineStore = useEngineStore()
const message = useAppMessage()
const dialog = useDialog()

const jsonText = ref('')
const importText = ref('')
const exportError = ref('')
const busy = ref(false)

async function buildExport() {
  busy.value = true
  exportError.value = ''
  try {
    const backup = buildSettingsBackup(preferenceStore.config, await getVersion())
    jsonText.value = JSON.stringify(backup, null, 2)
  } catch (error) {
    logger.warn('ConfigTransfer.export', error)
    jsonText.value = ''
    exportError.value = t('preferences.config-export-failed')
  } finally {
    busy.value = false
  }
}

async function copyJson() {
  if (!jsonText.value) return
  try {
    await writeAppClipboardText(jsonText.value)
    message.success(t('preferences.config-json-copied'))
  } catch (error) {
    logger.warn('ConfigTransfer.copy', error)
    message.error(t('task.checksum-copy-failed'))
  }
}

async function pasteImport() {
  try {
    const { readText } = await import('@tauri-apps/plugin-clipboard-manager')
    importText.value = (await readText()) || ''
  } catch (error) {
    logger.warn('ConfigTransfer.paste', error)
    message.error(t('task.paste-failed'))
  }
}

function importConfiguration() {
  if (!importText.value.trim()) {
    message.warning(t('preferences.config-import-empty'))
    return
  }
  let imported: AppConfig
  try {
    imported = parseSettingsBackup(importText.value)
  } catch (error) {
    logger.warn('ConfigTransfer.import', error)
    message.error(t('preferences.import-settings-invalid'))
    return
  }
  dialog.warning({
    title: t('preferences.import-settings'),
    content: t('preferences.import-settings-confirm'),
    positiveText: t('preferences.import-settings'),
    negativeText: t('app.cancel'),
    maskClosable: false,
    zIndex: 10500,
    onPositiveClick: async () => {
      busy.value = true
      try {
        const ok = await preferenceStore.replaceAndSave(imported)
        if (!ok) {
          message.error(t('preferences.import-settings-failed'))
          return
        }
        await invoke('replace_system_config', {
          config: buildSystemConfigFromAppConfig(imported, imported.dir),
        })
        message.success(t('preferences.import-settings-success'))
        emit('close')
        dialog.info({
          title: t('preferences.settings-imported'),
          content: t('preferences.import-settings-restart-confirm'),
          positiveText: t('preferences.restart-now'),
          negativeText: t('preferences.engine-restart-later'),
          maskClosable: false,
          zIndex: 10500,
          onPositiveClick: async () => {
            await engineStore.stop('appRelaunch')
            await relaunch()
          },
        })
      } catch (error) {
        logger.error('ConfigTransfer.import', error)
        message.error(t('preferences.import-settings-failed'))
      } finally {
        busy.value = false
      }
    },
  })
}

watch(
  () => props.show,
  (visible) => {
    if (!visible) return
    importText.value = ''
    void buildExport()
  },
)
</script>

<template>
  <NModal
    :show="show"
    :z-index="9000"
    transform-origin="center"
    @update:show="
      (value: boolean) => {
        if (!value) emit('close')
      }
    "
  >
    <NCard class="config-transfer" :bordered="false" closable @close="emit('close')">
      <template #header>{{ t('preferences.config-transfer-title') }}</template>

      <section class="config-transfer__section">
        <div class="config-transfer__heading">
          <strong>{{ t('preferences.config-export-title') }}</strong>
          <NButton size="small" :disabled="!jsonText" @click="copyJson">
            <template #icon
              ><NIcon><CopyOutline /></NIcon
            ></template>
            {{ t('preferences.config-copy-json') }}
          </NButton>
        </div>
        <NSpin :show="busy">
          <NInput :value="jsonText" type="textarea" :rows="5" readonly class="config-transfer__json" />
        </NSpin>
        <NAlert v-if="exportError" type="warning" :show-icon="false">{{ exportError }}</NAlert>
      </section>

      <section class="config-transfer__section">
        <div class="config-transfer__heading">
          <strong>{{ t('preferences.config-import-title') }}</strong>
        </div>
        <NInput
          v-model:value="importText"
          type="textarea"
          :rows="5"
          :placeholder="t('preferences.config-import-placeholder')"
          class="config-transfer__json"
        />
        <div class="config-transfer__actions">
          <NButton size="small" :disabled="busy" @click="pasteImport">
            {{ t('task.paste') }}
          </NButton>
          <NButton size="small" type="warning" :loading="busy" @click="importConfiguration">
            {{ t('preferences.import-settings') }}
          </NButton>
        </div>
      </section>
    </NCard>
  </NModal>
</template>

<style scoped>
.config-transfer {
  width: min(560px, calc(100vw - 28px));
  max-height: calc(100dvh - env(safe-area-inset-top, 0px) - env(safe-area-inset-bottom, 0px) - 24px);
  overflow-y: auto;
}
.config-transfer__section {
  display: grid;
  gap: 12px;
}
.config-transfer__section + .config-transfer__section {
  margin-top: 20px;
  padding-top: 18px;
  border-top: 1px solid var(--m3-outline-variant);
}
.config-transfer__heading,
.config-transfer__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  justify-content: space-between;
}
.config-transfer__actions {
  justify-content: flex-start;
}
.config-transfer__actions :deep(.n-button),
.config-transfer__heading :deep(.n-button) {
  min-height: 44px;
}
.config-transfer__json :deep(textarea) {
  font-family: var(--font-family-mono);
  font-size: var(--font-size-xs);
}
</style>
