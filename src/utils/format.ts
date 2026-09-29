const ACRONYMS: Record<string, string> = { md5: "MD5", usvfs: "USVFS" };

function sentenceCase(words: string): string {
  const out = words
    .toLowerCase()
    .split(" ")
    .map((w) => ACRONYMS[w] ?? w)
    .join(" ");
  return out.charAt(0).toUpperCase() + out.slice(1);
}

/** `anomalySkipVerify` -> `Anomaly skip verify` */
export function humanizeKey(key: string): string {
  return sentenceCase(
    key
      .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
      .replace(/[-_]+/g, " ")
      .trim(),
  );
}

/** `check-md5` -> `Check MD5` */
export function humanizeCommand(name: string): string {
  return sentenceCase(name.replace(/[-_]+/g, " ").trim());
}
