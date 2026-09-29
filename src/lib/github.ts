// Small public GitHub API helpers (unauthenticated; results are cached per session).

export interface ModOrganizerRelease {
  tag: string;
  prerelease: boolean;
}

interface GithubRelease {
  tag_name: string;
  prerelease: boolean;
  draft: boolean;
  assets: { name: string }[];
}

const MO_RELEASES_URL =
  "https://api.github.com/repos/ModOrganizer2/modorganizer/releases?per_page=50";

let moReleases: Promise<ModOrganizerRelease[]> | null = null;

/**
 * ModOrganizer2 releases usable by gamma-launcher, newest first: it downloads
 * `releases/download/<tag>/Mod.Organizer-<tag without v>.7z`, so the asset must exist.
 */
export function fetchModOrganizerReleases(): Promise<ModOrganizerRelease[]> {
  moReleases ??= fetch(MO_RELEASES_URL, { headers: { Accept: "application/vnd.github+json" } })
    .then((r) => {
      if (!r.ok) throw new Error(`GitHub API: HTTP ${r.status}`);
      return r.json() as Promise<GithubRelease[]>;
    })
    .then((releases) =>
      releases
        .filter(
          (r) =>
            !r.draft &&
            r.assets.some((a) => a.name === `Mod.Organizer-${r.tag_name.replace(/^v/, "")}.7z`),
        )
        .map((r) => ({ tag: r.tag_name, prerelease: r.prerelease })),
    )
    .catch((e) => {
      moReleases = null; // allow a retry later
      throw e;
    });
  return moReleases;
}
