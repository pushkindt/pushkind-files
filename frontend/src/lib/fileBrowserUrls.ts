import type { MountFileBrowserOptions } from "./fileBrowserModels";

export function normalizeBaseUrl(baseUrl: string) {
  if (!baseUrl) {
    return "";
  }

  return baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
}

export function withBaseUrl(baseUrl: string, path: string) {
  if (!path) {
    return normalizeBaseUrl(baseUrl);
  }

  if (
    path.startsWith("http://") ||
    path.startsWith("https://") ||
    path.startsWith("//")
  ) {
    return path;
  }

  const normalizedBase = normalizeBaseUrl(baseUrl);
  if (!normalizedBase) {
    return path;
  }

  if (path.startsWith("/")) {
    return `${normalizedBase}${path}`;
  }

  return `${normalizedBase}/${path}`;
}

export function buildBrowserUrl(baseUrl: string, path: string) {
  if (path.startsWith("/")) {
    return withBaseUrl(baseUrl, path);
  }
  if (path) {
    return withBaseUrl(
      baseUrl,
      `/files/browser?path=${encodeURIComponent(path)}`,
    );
  }

  return withBaseUrl(baseUrl, "/files/browser");
}

export function buildMainPageUrl(baseUrl: string, path: string) {
  if (path) {
    return withBaseUrl(baseUrl, `/?path=${encodeURIComponent(path)}`);
  }

  return withBaseUrl(baseUrl, "/");
}

export function buildUploadUrl(baseUrl: string, path: string) {
  if (path) {
    return withBaseUrl(
      baseUrl,
      `/files/upload?path=${encodeURIComponent(path)}`,
    );
  }

  return withBaseUrl(baseUrl, "/files/upload");
}

export function buildCreateFolderUrl(baseUrl: string, path: string) {
  if (path) {
    return withBaseUrl(
      baseUrl,
      `/folder/create?path=${encodeURIComponent(path)}`,
    );
  }

  return withBaseUrl(baseUrl, "/folder/create");
}

export function buildFileDownloadUrl(
  baseUrl: string,
  hubId: number,
  relativePath: string,
) {
  return withBaseUrl(
    baseUrl,
    `/upload/${hubId}/${encodeURIComponent(relativePath)}`,
  );
}

export function buildIamApiUrl(baseUrl: string) {
  return withBaseUrl(baseUrl, "/api/v1/iam");
}

export function buildEntriesApiUrl(baseUrl: string, path: string) {
  if (path) {
    return withBaseUrl(
      baseUrl,
      `/api/v1/files/entries?path=${encodeURIComponent(path)}`,
    );
  }

  return withBaseUrl(baseUrl, "/api/v1/files/entries");
}

export function resolveMountOptions(options?: MountFileBrowserOptions) {
  return {
    baseUrl: normalizeBaseUrl(options?.baseUrl ?? ""),
    historyMode: options?.historyMode ?? "managed",
  } as const;
}
