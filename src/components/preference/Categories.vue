<script setup lang="ts">
/** @fileoverview File category settings: auto-archive switch and category rules. */
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import { NButton, NForm, NFormItem, NInput, NSwitch, NText } from 'naive-ui'
import { usePreferenceStore } from '@/stores/preference'
import { usePreferenceForm } from '@/composables/usePreferenceForm'
import { useAppMessage } from '@/composables/useAppMessage'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { getVideoArchiveFolder, withVideoArchiveFolder } from '@shared/utils/autoArchive'
import FileCategoryManager from './FileCategoryManager.vue'
import PreferenceActionRegistration from './PreferenceActionRegistration.vue'
import PreferenceHintLabel from './PreferenceHintLabel.vue'
import type { FileCategory } from '@shared/types'

const { t } = useI18n()
const preferenceStore = usePreferenceStore()
const message = useAppMessage()

function buildForm() {
  return {
    fileCategoryEnabled: preferenceStore.config.fileCategoryEnabled,
    fileCategories: preferenceStore.config.fileCategories,
  }
}

const { form, isDirty, handleSave, handleReset, commitInstant } = usePreferenceForm({
  buildForm,
  transformForStore: (f) => ({
    fileCategoryEnabled: f.fileCategoryEnabled,
    fileCategories: f.fileCategories,
  }),
  beforeSave: async (f) => {
    try {
      await invoke('validate_file_categories', {
        categories: f.fileCategories,
        baseDir: preferenceStore.config.dir,
      })
      return true
    } catch (error) {
      message.error(getErrorMessage(error))
      return false
    }
  },
})

const showCategoryManager = ref(false)
const categoryBaseDir = computed(() => preferenceStore.config.dir)
const categorySummary = computed(() => {
  const categories = form.value.fileCategories
  const urlRuleCount = categories.reduce((total, category) => total + (category.urlPatterns?.length ?? 0), 0)
  return t('preferences.file-category-summary', { count: categories.length, url: urlRuleCount })
})
const videoArchiveFolder = computed({
  get: () => getVideoArchiveFolder(form.value.fileCategories),
  set: (value: string) => {
    form.value.fileCategories = withVideoArchiveFolder(form.value.fileCategories, value)
  },
})

async function handleCategoryManagerSave(categories: FileCategory[]) {
  const saved = await commitInstant({ fileCategories: categories }, () =>
    preferenceStore.updateAndSave({ fileCategories: categories }),
  )
  if (!saved) message.error(t('preferences.save-fail-message'))
}
</script>

<template>
  <div class="preference-form-wrapper">
    <div class="preference-form-scroll">
      <NForm label-placement="left" label-align="left" label-width="260px" size="small" class="form-preference">
        <NFormItem>
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.file-category-save')"
              :hint="t('preferences.file-category-auto-archive-hint')"
            />
          </template>
          <NSwitch v-model:value="form.fileCategoryEnabled" />
        </NFormItem>
        <NFormItem v-if="form.fileCategoryEnabled">
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.auto-archive-video-folder')"
              :hint="t('preferences.auto-archive-video-folder-hint')"
            />
          </template>
          <NInput v-model:value="videoArchiveFolder" class="pref-control-auto" />
        </NFormItem>
        <NFormItem :show-label="false">
          <div class="file-category-summary-row">
            <div class="file-category-summary-text">
              <span>{{ categorySummary }}</span>
              <NText depth="3">{{ t('preferences.file-category-manager-hint') }}</NText>
            </div>
            <NButton size="small" @click="showCategoryManager = true">
              {{ t('preferences.file-category-manage') }}
            </NButton>
          </div>
        </NFormItem>
      </NForm>
    </div>
    <PreferenceActionRegistration :is-dirty="isDirty" @save="handleSave" @discard="handleReset" />
    <FileCategoryManager
      v-model:show="showCategoryManager"
      :categories="form.fileCategories"
      :base-dir="categoryBaseDir"
      @save="handleCategoryManagerSave"
    />
  </div>
</template>

<style scoped>
.pref-control-auto {
  width: min(360px, 100%);
}

.file-category-summary-row {
  display: flex;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  flex-wrap: wrap;
}

.file-category-summary-text {
  display: grid;
  gap: 2px;
  min-width: 0;
}
</style>
