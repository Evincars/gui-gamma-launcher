// Directories shared by every command and remembered across restarts, so the
// user sets the Anomaly / GAMMA path once.

import { useLocalStorage } from "./useLocalStorage";

const DEFAULTS = { anomaly: "", gamma: "" };

export type SharedKey = keyof typeof DEFAULTS;

const shared = useLocalStorage("gamma-launcher.shared-paths", DEFAULTS);

export function isSharedKey(key: string): key is SharedKey {
  return key in DEFAULTS;
}

export function useSharedPaths() {
  return shared;
}
