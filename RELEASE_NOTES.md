# Rayload v1.0.1-beta2

## 中文

Rayload `v1.0.1-beta2` 继续完善 Android 17 体验，移除重复操作、精简任务卡片，
并补上可恢复的文件回收站。

### 更新内容

- 移除任务卡片中已废弃的“打开文件位置”按钮。
- 移除任务卡片左侧的竖条拖拽把手，手机端改为长按任务卡片排序。
- 将“全部”“进行中”“失败”“已完成”等任务分类文字居中。
- 新增侧边栏“回收站”入口，可查看、恢复或彻底删除移入 `.trash` 的文件。
- 重构手机任务页顶部：只保留标题、排序和“更多操作”，全局操作收纳到底部动作面板。
- 将“新建任务”改为右下角悬浮按钮，减少拥挤和误触。
- 修复长任务名在卡片中竖排，以及手机页面标题被工具栏挤没的问题。
- 保留首次启动语言选择、空状态 Logo、任务分享、权限引导和 SAF 文件夹选择修复。

### 安装包

```text
Rayload-v1.0.1-android17-arm64-release.apk
```

- 目标：Android 17 / API 37
- 最低：Android 7.0 / API 24
- 架构：`arm64-v8a`
- SHA-256：`d23e60ad6039ea2bee5a5bf38899da5be85432ca3085af5948d0066c093f428c`

Release APK 使用 Rayload 独立签名。升级前请确认设备上安装的是同一签名版本。

## English

Rayload `v1.0.1-beta2` polishes the Android 17 experience by removing redundant
actions, simplifying task cards, and adding a recoverable file Trash.

### What's new

- Removed the obsolete open-location button from task cards.
- Removed the visible drag rail and enabled long-press reordering on phones.
- Centered task-scope navigation labels.
- Added a sidebar Trash panel for restoring or permanently deleting `.trash`
  entries.
- Rebuilt the mobile task header around the title, sort, and a bottom action
  sheet, and moved New Task to a floating action button.
- Fixed clipped mobile headings and vertically collapsed long task names.
- Retained the language picker, empty-state logo, native sharing, permission
  guide, and SAF directory-selection fixes.

### Package

```text
Rayload-v1.0.1-android17-arm64-release.apk
```

- Target: Android 17 / API 37
- Minimum: Android 7.0 / API 24
- ABI: `arm64-v8a`
- SHA-256: `d23e60ad6039ea2bee5a5bf38899da5be85432ca3085af5948d0066c093f428c`

The Release APK uses the independent Rayload signing identity. Verify that the
installed build uses the same signing certificate before upgrading.

Rayload is an independent, unofficial port based on Rayburst by AnInsomniacy and
follows the original MIT license.
