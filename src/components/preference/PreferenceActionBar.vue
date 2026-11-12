<script setup lang="ts">
/** Persistent footer controls bound to the active preference form. */
import { useI18n } from 'vue-i18n'
import { NButton } from 'naive-ui'

withDefaults(defineProps<{ isDirty: boolean; isValid?: boolean }>(), { isValid: true })
const emit = defineEmits<{ save: []; discard: [] }>()
const { t } = useI18n()

const ENTER_MS = 380
const LEAVE_MS = 220
</script>

<template>
  <div class="form-actions">
    <!-- Shown only with unsaved changes: greyed-out buttons read as broken text (#723). -->
    <Transition name="dirty-actions" :duration="{ enter: ENTER_MS, leave: LEAVE_MS }">
      <div v-if="isDirty" class="dirty-actions">
        <NButton :type="isValid ? 'primary' : 'default'" :disabled="!isValid" @click="emit('save')">{{
          t('preferences.save')
        }}</NButton>
        <NButton class="secondary-action" type="error" ghost @click="emit('discard')">{{
          t('preferences.discard')
        }}</NButton>
      </div>
    </Transition>
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
.dirty-actions-enter-active {
  transition:
    opacity 380ms cubic-bezier(0.2, 0, 0, 1),
    transform 380ms cubic-bezier(0.2, 0, 0, 1);
}
.dirty-actions-leave-active {
  transition:
    opacity 220ms var(--dialog-ease-leave),
    transform 220ms var(--dialog-ease-leave);
}
.dirty-actions-enter-from,
.dirty-actions-leave-to {
  opacity: 0;
  transform: translateX(-16px);
}
@container footer (max-width: 640px) {
  .form-actions {
    --actions-gap: 8px;
  }
}
</style>
