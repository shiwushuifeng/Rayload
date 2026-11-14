/** @fileoverview Demo event bus: keeps fake snapshots away from native events. */
export type UnlistenFn = () => void

interface DemoEvent<T> {
  event: string
  id: number
  payload: T
}

type AnyHandler = (event: DemoEvent<unknown>) => void

const listeners = new Map<string, Set<AnyHandler>>()
let nextId = 1

/** Registers a listener on the in-memory demo bus. */
export async function listen<T>(event: string, handler: (event: DemoEvent<T>) => void): Promise<UnlistenFn> {
  const set = listeners.get(event) ?? new Set<AnyHandler>()
  const wrapped = handler as AnyHandler
  set.add(wrapped)
  listeners.set(event, set)
  return () => set.delete(wrapped)
}

/** Registers a one-shot listener. */
export async function once<T>(event: string, handler: (event: DemoEvent<T>) => void): Promise<UnlistenFn> {
  let stop: UnlistenFn | null = null
  const unlisten = await listen<T>(event, (value) => {
    stop?.()
    handler(value)
  })
  stop = unlisten
  return unlisten
}

/** Dispatches an event to every demo listener. */
export async function emit<T>(event: string, payload?: T): Promise<void> {
  for (const handler of listeners.get(event) ?? []) {
    handler({ event, id: nextId++, payload })
  }
}

/** Window-targeted events share the single demo bus. */
export async function emitTo<T>(_target: string, event: string, payload?: T): Promise<void> {
  await emit(event, payload)
}
