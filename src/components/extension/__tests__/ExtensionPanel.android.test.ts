import { beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import ExtensionPanel from '../ExtensionPanel.vue'

const mocks = vi.hoisted(() => ({
  push: vi.fn(),
  openUrl: vi.fn(),
  copyText: vi.fn(),
  success: vi.fn(),
  warning: vi.fn(),
  error: vi.fn(),
  config: { rpcSecret: 'test-rpc-secret' },
}))

vi.mock('@tauri-apps/plugin-opener', () => ({ openUrl: mocks.openUrl }))
vi.mock('vue-router', () => ({ useRouter: () => ({ push: mocks.push }) }))
vi.mock('vue-i18n', () => ({ useI18n: () => ({ t: (key: string) => key }) }))
vi.mock('@/composables/usePlatform', () => ({ usePlatform: () => ({ isAndroid: { value: true } }) }))
vi.mock('@/stores/preference', () => ({ usePreferenceStore: () => ({ config: mocks.config }) }))
vi.mock('@shared/utils', () => ({ writeAppClipboardText: mocks.copyText }))
vi.mock('@/composables/useAppMessage', () => ({
  useAppMessage: () => ({
    success: mocks.success,
    warning: mocks.warning,
    error: mocks.error,
  }),
}))
vi.mock('naive-ui', () => ({
  NModal: {
    props: ['show'],
    template: '<div v-if="show"><slot /></div>',
  },
  NButton: {
    emits: ['click'],
    template: '<button type="button" @click="$emit(\'click\')"><slot name="icon" /><slot /></button>',
  },
  NIcon: { template: '<span><slot /></span>' },
}))

beforeEach(() => {
  vi.clearAllMocks()
})

describe('Android extension guidance', () => {
  it('keeps extension instructions inside the app and copies the RPC secret', async () => {
    const wrapper = mount(ExtensionPanel, {
      props: { show: true },
    })

    expect(wrapper.text()).toContain('extension.android-title')
    expect(wrapper.text()).toContain('extension.android-description')
    expect(wrapper.text()).toContain('http://127.0.0.1:6800/jsonrpc')
    expect(wrapper.findAll('a')).toHaveLength(0)

    const copy = wrapper.findAll('button').find((button) => button.text() === 'extension.android-copy-rpc-secret')
    expect(copy).toBeDefined()
    await copy!.trigger('click')
    await flushPromises()

    expect(mocks.copyText).toHaveBeenCalledWith('test-rpc-secret')
    expect(mocks.success).toHaveBeenCalledWith('extension.android-rpc-secret-copied')
    expect(mocks.openUrl).not.toHaveBeenCalled()
    wrapper.unmount()
  })
})
