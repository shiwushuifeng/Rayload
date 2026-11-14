<script setup lang="ts">
/** Persistent footer controls bound to the active preference form. */
import { useI18n } from 'vue-i18n'
import { NButton } from 'naive-ui'

withDefaults(defineProps<{ isDirty: boolean; isValid?: boolean }>(), { isValid: true })
const emit = defineEmits<{ save: []; discard: [] }>()
const { t } = useI18n()
</script>

<template>
  <div class="form-actions">
    <!-- Always visible so the actions do not jump in and out; disabled until
         the form has unsaved changes. -->
    <div class="dirty-actions">
      <NButton
        :type="isDirty && isValid ? 'primary' : 'default'"
        :disabled="!isDirty || !isValid"
        @click="emit('save')"
        >{{ t('preferences.save') }}</NButton
      >
      <NButton class="secondary-action" type="error" ghost :disabled="!isDirty" @click="emit('discard')">{{
        t('preferences.discard')
      }}</NButton>
    </div>
  </div>
</template>

<style scoped>
.form-actions,
.dirty-actions {
  display: flex;
  align-items: center;
  gap: var(--actions-gap);
  min-width: 0;
}
.form-actions {
  --actions-gap: 12px;
  min-height: 46px;
}
.dirty-actions {
  flex-shrink: 1;
}
.dirty-actions > :first-child {
  min-width: 0;
  flex-shrink: 1;
}
/* Clipped sideways only: the button's line height is 1, and CJK glyphs drawn in
   a fallback font taller than the Latin one would lose their top row (#723). */
.form-actions :deep(.n-button__content) {
  display: block;
  overflow: clip visible;
  text-overflow: ellipsis;
}
@container footer (max-width: 640px) {
  .form-actions {
    --actions-gap: 8px;
  }
}
</style>
