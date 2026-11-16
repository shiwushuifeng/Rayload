/** @fileoverview Vue Router configuration with task and preference routes. */
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/layouts/MainLayout.vue'),
      children: [
        {
          path: '',
          redirect: '/task/all',
        },
        {
          path: '/lan',
          name: 'lan-devices',
          component: () => import('@/views/LanDevicesView.vue'),
        },
        {
          path: '/plugins',
          name: 'plugins',
          component: () => import('@/views/PluginsView.vue'),
        },
        {
          path: '/statistics',
          name: 'statistics',
          component: () => import('@/views/StatisticsView.vue'),
        },
        {
          path: '/task/:status(all|progress|failed|completed)?',
          name: 'task',
          component: () => import('@/views/TaskView.vue'),
          props: true,
        },
        {
          path: '/preference',
          name: 'preference',
          component: () => import('@/views/PreferenceView.vue'),
          children: [
            {
              path: 'general',
              alias: '',
              name: 'preference-general',
              component: () => import('@/components/preference/General.vue'),
            },
            {
              path: 'downloader',
              name: 'preference-downloader',
              component: () => import('@/components/preference/Downloader.vue'),
            },
            {
              path: 'categories',
              name: 'preference-categories',
              component: () => import('@/components/preference/Categories.vue'),
            },
            {
              path: 'downloads',
              redirect: { name: 'preference-downloader', query: { section: 'downloads' } },
            },
            {
              path: 'bt',
              redirect: { name: 'preference-downloader', query: { section: 'bt' } },
            },
            {
              path: 'ed2k',
              redirect: { name: 'preference-downloader', query: { section: 'ed2k' } },
            },
            {
              path: 'network',
              redirect: { name: 'preference-downloader', query: { section: 'network' } },
            },
            {
              path: 'permissions',
              name: 'preference-permissions',
              component: () => import('@/components/preference/Permissions.vue'),
            },
            {
              path: 'lan',
              name: 'preference-lan',
              component: () => import('@/views/LanDevicesView.vue'),
            },
            {
              path: 'plugins',
              name: 'preference-plugins',
              component: () => import('@/views/PluginsView.vue'),
            },
            {
              path: 'advanced',
              name: 'preference-advanced',
              component: () => import('@/components/preference/Advanced.vue'),
            },
          ],
        },
      ],
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: '/',
    },
  ],
})

export default router
