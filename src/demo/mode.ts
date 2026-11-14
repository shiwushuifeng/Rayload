/** Demo builds render fixed promotional data instead of the real engine. */
export const DEMO_MODE =
  import.meta.env.VITE_RAYLOAD_DEMO === '1' ||
  (import.meta.env.DEV && new URLSearchParams(window.location.search).has('demo'))
