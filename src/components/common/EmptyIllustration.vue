<script setup lang="ts">
/** Minimal line-art empty states using the Rayload violet and warm-gold palette. */
import { useId } from 'vue'

defineProps<{ variant: 'tasks' | 'trash' | 'devices' }>()

const uid = useId().replace(/[^a-zA-Z0-9_-]/g, '')
</script>

<template>
  <svg class="empty-illustration" viewBox="0 0 220 168" role="presentation" aria-hidden="true">
    <defs>
      <radialGradient :id="`empty-glow-${uid}`" cx="50%" cy="46%" r="54%">
        <stop offset="0" stop-color="var(--m3-primary)" stop-opacity="0.16" />
        <stop offset="1" stop-color="var(--m3-primary)" stop-opacity="0" />
      </radialGradient>
      <linearGradient :id="`empty-sheen-${uid}`" x1="0" y1="0" x2="1" y2="1">
        <stop offset="0" stop-color="var(--illustration-gold)" stop-opacity="0.9" />
        <stop offset="1" stop-color="var(--illustration-gold)" stop-opacity="0.25" />
      </linearGradient>
    </defs>

    <ellipse cx="110" cy="86" rx="92" ry="72" :fill="`url(#empty-glow-${uid})`" />

    <g v-if="variant === 'tasks'" class="empty-illustration__tasks">
      <circle
        cx="110"
        cy="84"
        r="62"
        fill="none"
        stroke="var(--m3-primary)"
        stroke-opacity="0.24"
        stroke-width="1.5"
        stroke-dasharray="3 9"
      />
      <rect
        x="53"
        y="42"
        width="114"
        height="78"
        rx="18"
        fill="var(--m3-surface-container-lowest)"
        stroke="var(--m3-primary)"
        stroke-width="2.5"
      />
      <rect x="68" y="57" width="48" height="6" rx="3" fill="var(--m3-primary)" opacity="0.32" />
      <rect x="68" y="70" width="84" height="5" rx="2.5" fill="var(--m3-primary)" opacity="0.14" />
      <circle cx="110" cy="94" r="16" fill="var(--m3-primary-container)" />
      <path
        d="M110 87v14m0 0-5.5-5.5M110 101l5.5-5.5"
        fill="none"
        stroke="var(--m3-on-primary-container)"
        stroke-width="2.4"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <path d="M173 46l3.2 7.4L184 57l-7.8 3.4L173 68l-3.4-7.6L162 57l7.6-3.6z" :fill="`url(#empty-sheen-${uid})`" />
      <circle cx="43" cy="110" r="3.2" fill="var(--illustration-gold)" opacity="0.7" />
      <circle cx="183" cy="101" r="2.4" fill="var(--m3-primary)" opacity="0.35" />
    </g>

    <g v-else-if="variant === 'trash'" class="empty-illustration__trash">
      <circle
        cx="110"
        cy="86"
        r="61"
        fill="none"
        stroke="var(--m3-primary)"
        stroke-opacity="0.22"
        stroke-width="1.5"
        stroke-dasharray="2 10"
      />
      <path d="M86 58h48" fill="none" stroke="var(--m3-primary)" stroke-width="3" stroke-linecap="round" />
      <path
        d="M98 58v-6a4 4 0 0 1 4-4h16a4 4 0 0 1 4 4v6"
        fill="none"
        stroke="var(--m3-primary)"
        stroke-width="2.4"
        stroke-linecap="round"
      />
      <path
        d="M91 62h38l-3.4 48.5a8 8 0 0 1-8 7.5H102.4a8 8 0 0 1-8-7.5z"
        fill="var(--m3-surface-container-lowest)"
        stroke="var(--m3-primary)"
        stroke-width="2.5"
        stroke-linejoin="round"
      />
      <path
        d="M103 78v24M110 78v24M117 78v24"
        fill="none"
        stroke="var(--m3-primary)"
        stroke-opacity="0.36"
        stroke-width="2.2"
        stroke-linecap="round"
      />
      <path
        d="M166 52l2.6 6.1 6.4 2.6-6.4 2.7-2.6 6.1-2.6-6.1-6.4-2.7 6.4-2.6z"
        fill="var(--illustration-gold)"
        opacity="0.86"
      />
      <circle cx="55" cy="82" r="3" fill="var(--illustration-gold)" opacity="0.64" />
    </g>

    <g v-else class="empty-illustration__devices">
      <rect
        x="38"
        y="52"
        width="58"
        height="92"
        rx="12"
        fill="var(--m3-surface-container-lowest)"
        stroke="var(--m3-primary)"
        stroke-width="2.5"
      />
      <path d="M58 58h18" stroke="var(--m3-primary)" stroke-width="2" stroke-linecap="round" opacity="0.5" />
      <rect
        x="132"
        y="68"
        width="52"
        height="76"
        rx="10"
        fill="var(--m3-surface-container-lowest)"
        stroke="var(--m3-primary)"
        stroke-width="2.5"
      />
      <circle cx="158" cy="136" r="2.4" fill="var(--m3-primary)" opacity="0.55" />
      <path
        d="M96 94h36"
        fill="none"
        stroke="var(--illustration-gold)"
        stroke-width="2.4"
        stroke-linecap="round"
        stroke-dasharray="3 7"
      />
      <circle cx="96" cy="94" r="4" fill="var(--illustration-gold)" />
      <circle cx="132" cy="94" r="4" fill="var(--illustration-gold)" />
      <path
        d="M170 44l2.4 5.6 5.9 2.4-5.9 2.5-2.4 5.5-2.4-5.5-5.9-2.5 5.9-2.4z"
        fill="var(--illustration-gold)"
        opacity="0.8"
      />
    </g>
  </svg>
</template>

<style scoped>
.empty-illustration {
  display: block;
  width: 100%;
  height: auto;
  overflow: visible;
}
</style>
