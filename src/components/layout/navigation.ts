/** Shared navigation labels and destinations. */
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { ListOutline, PlayOutline, AlertCircleOutline, CheckmarkDoneOutline } from '@vicons/ionicons5'
import type { I18nKey } from '@shared/i18nTypes'

export function useTaskDestinations() {
  const { t } = useI18n()
  return computed(
    () =>
      [
        { key: 'all', label: t('task.scope-all'), icon: ListOutline },
        { key: 'progress', label: t('task.scope-progress'), icon: PlayOutline },
        { key: 'failed', label: t('task.scope-failed'), icon: AlertCircleOutline },
        { key: 'completed', label: t('task.scope-completed'), icon: CheckmarkDoneOutline },
      ] as const,
  )
}

export const preferenceDestinations = [
  'general',
  'downloader',
  'categories',
  'lan',
  'permissions',
  'advanced',
  'checksum',
  'plugins',
] as const

export const preferenceGroups = [
  {
    key: 'account',
    labelKey: 'preferences.group-account',
    hintKey: 'preferences.group-account-hint',
    items: ['lan', 'permissions'],
  },
  {
    key: 'downloads',
    labelKey: 'preferences.group-download',
    hintKey: 'preferences.group-download-hint',
    items: ['downloader', 'categories'],
  },
  {
    key: 'appearance',
    labelKey: 'preferences.group-appearance',
    hintKey: 'preferences.group-appearance-hint',
    items: ['general'],
  },
  {
    key: 'advanced',
    labelKey: 'preferences.group-advanced',
    hintKey: 'preferences.group-advanced-hint',
    items: ['advanced', 'checksum', 'plugins'],
  },
] as const satisfies ReadonlyArray<{
  key: string
  labelKey: I18nKey
  hintKey: I18nKey
  items: readonly (typeof preferenceDestinations)[number][]
}>
