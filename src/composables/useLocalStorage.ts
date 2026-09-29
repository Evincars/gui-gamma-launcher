import { reactive, watch } from "vue";

function readStored(key: string): Record<string, unknown> {
  try {
    const parsed: unknown = JSON.parse(localStorage.getItem(key) ?? "null");
    return parsed && typeof parsed === "object" ? (parsed as Record<string, unknown>) : {};
  } catch {
    return {};
  }
}

/**
 * Reactive object persisted to `localStorage` under `key`. Only keys present in
 * `defaults` with a matching type are restored, so stale/corrupt data is ignored.
 */
export function useLocalStorage<T extends Record<string, string | number | boolean>>(
  key: string,
  defaults: T,
): T {
  const stored = readStored(key);
  const state = reactive({ ...defaults }) as T;
  for (const k of Object.keys(defaults) as (keyof T)[]) {
    if (typeof stored[k as string] === typeof defaults[k]) state[k] = stored[k as string] as T[keyof T];
  }

  watch(
    state,
    () => {
      try {
        localStorage.setItem(key, JSON.stringify(state));
      } catch {
        // Storage full or unavailable — settings just won't persist.
      }
    },
    { deep: true },
  );
  return state;
}
