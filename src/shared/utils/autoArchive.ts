/** @fileoverview Helpers for the archive folders used by automatic archiving. */
import sanitizeFilename from 'sanitize-filename'
import { BUILTIN_CATEGORY_TEMPLATES } from '@shared/constants'
import type { FileCategory } from '@shared/types'

export const VIDEO_CATEGORY_LABEL = 'file-category-videos'
export const DEFAULT_VIDEO_ARCHIVE_FOLDER = 'Video'
const MAX_FOLDER_LENGTH = 64

/** Keeps the archive folder a single relative directory name. */
export function normalizeArchiveFolderName(value: string, fallback = DEFAULT_VIDEO_ARCHIVE_FOLDER): string {
  const cleaned = value.trim().replace(/[\\/]+/g, '-')
  const safe = sanitizeFilename(cleaned).trim()
  if (!safe || safe === '.' || safe === '..') return fallback
  return safe.slice(0, MAX_FOLDER_LENGTH)
}

/** Returns the configured folder of a built-in category, or the template default. */
export function getBuiltInArchiveFolder(categories: readonly FileCategory[], label: string): string {
  const template = BUILTIN_CATEGORY_TEMPLATES.find((item) => item.label === label)
  const fallback = template?.subdirName ?? DEFAULT_VIDEO_ARCHIVE_FOLDER
  const match = categories.find((category) => category.label === label && category.directoryMode === 'relative')
  return match?.directory?.trim() || fallback
}

export interface ArchiveCategorySettings {
  /** Labels of built-in categories that should keep archiving files. */
  enabledLabels: readonly string[]
  /** Folder overrides keyed by built-in category label. */
  folders: Readonly<Record<string, string>>
}

/** Applies onboarding archive choices to built-in categories while keeping custom rules. */
export function applyArchiveCategorySettings(
  categories: readonly FileCategory[],
  settings: ArchiveCategorySettings,
): FileCategory[] {
  const enabled = new Set(settings.enabledLabels)
  const kept = categories.filter((category) => !category.builtIn || enabled.has(category.label))
  const result = kept.map((category) => {
    if (!category.builtIn || !enabled.has(category.label)) return category
    const template = BUILTIN_CATEGORY_TEMPLATES.find((item) => item.label === category.label)
    if (!template) return category
    const draft = settings.folders[category.label] ?? category.directory
    return {
      ...category,
      directory: normalizeArchiveFolderName(draft, template.subdirName),
      directoryMode: 'relative' as const,
    }
  })
  const missing = BUILTIN_CATEGORY_TEMPLATES.filter(
    (template) => enabled.has(template.label) && !result.some((category) => category.label === template.label),
  )
  for (const template of missing) {
    result.push({
      label: template.label,
      extensions: [...template.extensions],
      directory: normalizeArchiveFolderName(
        settings.folders[template.label] ?? template.subdirName,
        template.subdirName,
      ),
      directoryMode: 'relative',
      builtIn: true,
    })
  }
  return result
}

/** Returns the configured video folder, or the product default. */
export function getVideoArchiveFolder(categories: readonly FileCategory[]): string {
  const video = categories.find(
    (category) => category.label === VIDEO_CATEGORY_LABEL && category.directoryMode === 'relative',
  )
  return video?.directory?.trim() || DEFAULT_VIDEO_ARCHIVE_FOLDER
}

/** Returns categories with the built-in video folder replaced. */
export function withVideoArchiveFolder(categories: readonly FileCategory[], folder: string): FileCategory[] {
  const directory = normalizeArchiveFolderName(folder)
  let replaced = false
  const result = categories.map((category) => {
    if (replaced || category.label !== VIDEO_CATEGORY_LABEL) return category
    replaced = true
    return { ...category, directory, directoryMode: 'relative' as const }
  })
  if (replaced) return result
  return [
    ...result,
    {
      label: VIDEO_CATEGORY_LABEL,
      extensions: ['mp4', 'mkv', 'avi', 'mov', 'wmv', 'flv', 'webm', 'ts', 'm4v', 'rmvb'],
      directory,
      directoryMode: 'relative',
      builtIn: true,
    },
  ]
}
