/** @fileoverview Maps a task filename to a compact file-type icon and tone. */
import { computed, type Component, type Ref } from 'vue'
import {
  ArchiveOutline,
  CodeSlashOutline,
  CubeOutline,
  DocumentOutline,
  ImageOutline,
  LinkOutline,
  MusicalNotesOutline,
  VideocamOutline,
} from '@vicons/ionicons5'

export type TaskFileTone = 'video' | 'audio' | 'image' | 'archive' | 'code' | 'disk' | 'link' | 'file'

interface TaskFileIcon {
  icon: Component
  tone: TaskFileTone
}

const EXTENSION_GROUPS: Array<{ tone: TaskFileTone; icon: Component; extensions: readonly string[] }> = [
  {
    tone: 'video',
    icon: VideocamOutline,
    extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'm4v', 'mpg', 'mpeg', 'ts', 'rmvb', 'wmv'],
  },
  { tone: 'audio', icon: MusicalNotesOutline, extensions: ['mp3', 'flac', 'wav', 'aac', 'm4a', 'ogg', 'opus', 'wma'] },
  {
    tone: 'image',
    icon: ImageOutline,
    extensions: ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp', 'svg', 'heic', 'avif'],
  },
  { tone: 'archive', icon: ArchiveOutline, extensions: ['zip', 'rar', '7z', 'tar', 'gz', 'bz2', 'xz', 'zst'] },
  {
    tone: 'code',
    icon: CodeSlashOutline,
    extensions: [
      'js',
      'ts',
      'vue',
      'py',
      'java',
      'kt',
      'c',
      'cpp',
      'h',
      'rs',
      'go',
      'html',
      'css',
      'json',
      'xml',
      'sh',
    ],
  },
  { tone: 'disk', icon: CubeOutline, extensions: ['iso', 'img', 'apk', 'exe', 'dmg', 'bin', 'msi'] },
]

function extensionOf(name: string): string {
  const clean = name.split(/[?#]/u)[0] ?? ''
  const dot = clean.lastIndexOf('.')
  return dot >= 0 ? clean.slice(dot + 1).toLowerCase() : ''
}

export function useTaskFileIcon(name: Ref<string>) {
  return computed<TaskFileIcon>(() => {
    const value = name.value
    if (/^magnet:/iu.test(value) || /^ed2k:/iu.test(value)) return { icon: LinkOutline, tone: 'link' }
    const extension = extensionOf(value)
    const group = EXTENSION_GROUPS.find((candidate) => candidate.extensions.includes(extension))
    return group ? { icon: group.icon, tone: group.tone } : { icon: DocumentOutline, tone: 'file' }
  })
}
