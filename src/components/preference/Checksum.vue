<script setup lang="ts">
/** @fileoverview Checksum preference tab: algorithms, auto verification and skip rules. */
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  NButton,
  NCheckbox,
  NCheckboxGroup,
  NDivider,
  NForm,
  NFormItem,
  NIcon,
  NInput,
  NSelect,
  NSpace,
  NSwitch,
  NText,
} from 'naive-ui'
import { TrashOutline } from '@vicons/ionicons5'
import { CHECKSUM_ALGORITHMS, type ChecksumAlgorithm } from '@shared/types'
import { usePreferenceStore } from '@/stores/preference'
import { usePreferenceForm } from '@/composables/usePreferenceForm'
import { useAppMessage } from '@/composables/useAppMessage'
import { clearChecksumCache } from '@/api/checksum'
import { logger } from '@shared/logger'
import { getErrorMessage } from '@shared/utils/errorMessage'
import { buildChecksumForm, buildChecksumSystemConfig, createChecksumRule } from '@/composables/useChecksumPreference'
import { usePreferenceLabelWidth } from '@/composables/usePreferenceLabelWidth'
import PreferenceActionRegistration from './PreferenceActionRegistration.vue'
import PreferenceHintLabel from './PreferenceHintLabel.vue'

const { t } = useI18n()
const { preferenceLabelWidth } = usePreferenceLabelWidth()
const preferenceStore = usePreferenceStore()
const message = useAppMessage()

const ALGORITHM_LABELS: Record<ChecksumAlgorithm, string> = {
  md5: 'MD5',
  sha1: 'SHA-1',
  sha256: 'SHA-256',
  sha512: 'SHA-512',
  blake3: 'BLAKE3',
  sm3: 'SM3',
  crc32: 'CRC32',
}
const algorithmOptions = CHECKSUM_ALGORITHMS.map((algorithm) => ({
  value: algorithm,
  label: ALGORITHM_LABELS[algorithm],
}))
const categoryOptions = computed(() =>
  preferenceStore.config.fileCategories.map((category) => ({ value: category.label, label: category.label })),
)
const ruleModeOptions = computed(() => [
  { value: 'deny', label: t('preferences.checksum-rule-deny') },
  { value: 'allow', label: t('preferences.checksum-rule-allow') },
])
const clearingCache = ref(false)

const { form, isDirty, handleSave, handleReset } = usePreferenceForm({
  buildForm: () => buildChecksumForm(preferenceStore.config),
  buildSystemConfig: buildChecksumSystemConfig,
})

function addRule() {
  form.value.checksumRules = [...form.value.checksumRules, createChecksumRule(form.value.checksumRules.length)]
}

function removeRule(index: number) {
  form.value.checksumRules = form.value.checksumRules.filter((_, position) => position !== index)
}

async function handleClearCache() {
  clearingCache.value = true
  try {
    const removed = await clearChecksumCache()
    message.success(t('preferences.checksum-cache-cleared', { count: removed }))
  } catch (error) {
    logger.warn('Checksum.clearCache', getErrorMessage(error))
    message.error(t('preferences.checksum-cache-clear-failed'))
  } finally {
    clearingCache.value = false
  }
}
</script>

<template>
  <div class="preference-form-wrapper">
    <div class="preference-form-scroll">
      <NForm
        label-placement="left"
        label-align="left"
        :label-width="preferenceLabelWidth"
        size="small"
        class="form-preference"
      >
        <NDivider title-placement="left">{{ t('preferences.checksum-title') }}</NDivider>
        <NFormItem :show-label="false">
          <NText depth="3">{{ t('preferences.checksum-hint') }}</NText>
        </NFormItem>
        <NFormItem>
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.checksum-algorithms')"
              :hint="t('preferences.checksum-algorithms-hint')"
            />
          </template>
          <NCheckboxGroup v-model:value="form.checksumAlgorithms">
            <NSpace>
              <NCheckbox
                v-for="algorithm in algorithmOptions"
                :key="algorithm.value"
                :value="algorithm.value"
                :label="algorithm.label"
              />
            </NSpace>
          </NCheckboxGroup>
        </NFormItem>
        <NFormItem>
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.checksum-auto-verify')"
              :hint="t('preferences.checksum-auto-verify-hint')"
            />
          </template>
          <NSwitch v-model:value="form.checksumAutoVerify" />
        </NFormItem>
        <NFormItem v-if="form.checksumAutoVerify">
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.checksum-categories')"
              :hint="t('preferences.checksum-categories-hint')"
            />
          </template>
          <NCheckboxGroup v-model:value="form.checksumAutoVerifyCategories">
            <NSpace>
              <NCheckbox
                v-for="category in categoryOptions"
                :key="category.value"
                :value="category.value"
                :label="category.label"
              />
            </NSpace>
          </NCheckboxGroup>
        </NFormItem>
        <NFormItem>
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.checksum-online-lookup')"
              :hint="t('preferences.checksum-online-lookup-hint')"
            />
          </template>
          <NSwitch v-model:value="form.checksumOnlineLookup" />
        </NFormItem>
        <NFormItem>
          <template #label>
            <PreferenceHintLabel
              :label="t('preferences.checksum-rules')"
              :hint="t('preferences.checksum-rules-hint')"
            />
          </template>
          <div class="checksum-rules">
            <div v-for="(rule, index) in form.checksumRules" :key="rule.id" class="checksum-rule-row">
              <NInput
                v-model:value="rule.pattern"
                :placeholder="t('preferences.checksum-rule-pattern')"
                class="checksum-rule-pattern"
              />
              <NCheckbox v-model:checked="rule.regex" :label="t('preferences.checksum-rule-regex')" />
              <NSelect v-model:value="rule.mode" :options="ruleModeOptions" class="checksum-rule-mode" />
              <NButton quaternary :title="t('app.delete')" @click="removeRule(index)">
                <template #icon>
                  <NIcon :size="14"><TrashOutline /></NIcon>
                </template>
              </NButton>
            </div>
            <NButton dashed size="small" @click="addRule">{{ t('preferences.checksum-rule-add') }}</NButton>
          </div>
        </NFormItem>
        <NFormItem :label="t('preferences.checksum-cache-clear')">
          <NButton secondary :loading="clearingCache" @click="handleClearCache">
            {{ t('preferences.checksum-cache-clear') }}
          </NButton>
        </NFormItem>
      </NForm>
    </div>
    <PreferenceActionRegistration :is-dirty="isDirty" :is-valid="true" @save="handleSave" @discard="handleReset" />
  </div>
</template>

<style scoped>
.checksum-rules {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
  min-width: 0;
}
.checksum-rule-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  min-width: 0;
}
.checksum-rule-pattern {
  flex: 1 1 220px;
  min-width: 0;
}
.checksum-rule-mode {
  flex: 0 0 120px;
}
</style>
