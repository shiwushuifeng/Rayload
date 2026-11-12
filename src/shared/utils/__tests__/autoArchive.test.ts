import { describe, expect, it } from 'vitest'
import { buildDefaultCategories } from '@shared/constants'
import {
  DEFAULT_VIDEO_ARCHIVE_FOLDER,
  applyArchiveCategorySettings,
  getBuiltInArchiveFolder,
  getVideoArchiveFolder,
  normalizeArchiveFolderName,
  withVideoArchiveFolder,
} from '../autoArchive'

describe('auto archive folder helpers', () => {
  it('defaults to Video and reads the built-in video category', () => {
    const categories = buildDefaultCategories()
    expect(getVideoArchiveFolder(categories)).toBe(DEFAULT_VIDEO_ARCHIVE_FOLDER)
    expect(DEFAULT_VIDEO_ARCHIVE_FOLDER).toBe('Video')
  })

  it('replaces only the video category and keeps it relative', () => {
    const categories = withVideoArchiveFolder(buildDefaultCategories(), '影视')
    expect(categories.find((category) => category.label === 'file-category-videos')?.directory).toBe('影视')
    expect(categories.find((category) => category.label === 'file-category-music')?.directory).toBe('Music')
  })

  it('normalizes unsafe folder names back to a single segment', () => {
    expect(normalizeArchiveFolderName(' ../../Video ')).toBe('..-..-Video')
    expect(normalizeArchiveFolderName('影视/电影')).toBe('影视-电影')
    expect(normalizeArchiveFolderName('   ')).toBe('Video')
  })

  it('adds a video category when the user removed it', () => {
    const categories = withVideoArchiveFolder([], 'Clips')
    expect(categories).toHaveLength(1)
    expect(categories[0]).toMatchObject({ label: 'file-category-videos', directory: 'Clips', builtIn: true })
  })
})

describe('onboarding archive category settings', () => {
  it('reads the folder of any built-in category', () => {
    const categories = withVideoArchiveFolder(buildDefaultCategories(), '影视')
    expect(getBuiltInArchiveFolder(categories, 'file-category-videos')).toBe('影视')
    expect(getBuiltInArchiveFolder(categories, 'file-category-music')).toBe('Music')
  })

  it('drops disabled built-in categories and keeps custom rules', () => {
    const custom = {
      label: 'Podcast',
      extensions: ['mp3'],
      directory: 'Audio/Podcast',
      directoryMode: 'relative' as const,
    }
    const result = applyArchiveCategorySettings(buildDefaultCategories(), {
      enabledLabels: ['file-category-videos', 'file-category-images'],
      folders: {},
    })
    expect(result.map((category) => category.label)).toEqual(['file-category-videos', 'file-category-images'])
    expect(
      applyArchiveCategorySettings([...buildDefaultCategories(), custom], {
        enabledLabels: ['file-category-music'],
        folders: {},
      }).some((category) => category.label === 'Podcast'),
    ).toBe(true)
  })

  it('applies renamed folders and restores removed built-ins', () => {
    const result = applyArchiveCategorySettings([], {
      enabledLabels: ['file-category-videos', 'file-category-music'],
      folders: {
        'file-category-videos': '影视',
        'file-category-music': '音乐/合辑',
      },
    })
    expect(result.find((category) => category.label === 'file-category-videos')?.directory).toBe('影视')
    expect(result.find((category) => category.label === 'file-category-music')?.directory).toBe('音乐-合辑')
    expect(result.every((category) => category.directoryMode === 'relative')).toBe(true)
  })
})
