import { describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import PreferenceActionBar from '../PreferenceActionBar.vue'

vi.mock('vue-i18n', () => ({ useI18n: () => ({ t: (key: string) => key }) }))
vi.mock('naive-ui', () => ({
  NButton: { props: ['disabled'], template: '<button :disabled="disabled"><slot /></button>' },
}))
describe('shared preference footer', () => {
  it('keeps save and discard owned by the form while rendering in the footer', async () => {
    const wrapper = mount(PreferenceActionBar, { props: { isDirty: true } })
    await flushPromises()
    expect(wrapper.find('.form-actions').exists()).toBe(true)
    const buttons = wrapper.element.querySelectorAll('button')
    buttons[0].click()
    buttons[1].click()
    expect(wrapper.emitted('save')).toHaveLength(1)
    expect(wrapper.emitted('discard')).toHaveLength(1)
    // Without changes the footer stays empty.
    await wrapper.setProps({ isDirty: false })
    expect(wrapper.findAll('button')).toHaveLength(0)
    wrapper.unmount()
  })

  it('retains validation on the save action', async () => {
    const wrapper = mount(PreferenceActionBar, { props: { isDirty: true, isValid: false } })
    await flushPromises()
    expect(wrapper.element.querySelector('button')?.disabled).toBe(true)
    wrapper.unmount()
  })
})
