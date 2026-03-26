import type {
  FileBrowserApiResponse,
  FileBrowserEntry,
  FileBrowserViewModel,
  FieldErrors,
  FilesPageBootstrapData,
  FilesShellData,
  MutationResult,
  UserMenuItem,
} from "./fileBrowserModels";
import {
  buildCreateFolderUrl,
  buildEntriesApiUrl,
  buildIamApiUrl,
  buildUploadUrl,
  withBaseUrl,
} from "./fileBrowserUrls";

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function readString(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (typeof value !== "string") {
    throw new Error(`Invalid API response: expected string at ${key}.`);
  }

  return value;
}

function readBoolean(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (typeof value !== "boolean") {
    throw new Error(`Invalid API response: expected boolean at ${key}.`);
  }

  return value;
}

function readOptionalString(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (value == null) {
    return undefined;
  }
  if (typeof value !== "string") {
    throw new Error(`Invalid API response: expected string at ${key}.`);
  }

  return value;
}

function readNumber(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (typeof value !== "number") {
    throw new Error(`Invalid API response: expected number at ${key}.`);
  }

  return value;
}

async function fetchJson(url: string) {
  const response = await fetch(url, {
    credentials: "include",
  });

  if (!response.ok) {
    if (response.status === 400) {
      throw new Error("Недопустимый путь.");
    }
    if (response.status === 401) {
      throw new Error("Недостаточно прав для доступа к файлам.");
    }
    throw new Error(`Request failed with status ${response.status}.`);
  }

  return response.json();
}

export const browserLocation = {
  assign(url: string) {
    window.location.assign(url);
  },
};

async function readResponseJson(response: Response) {
  const contentType = response.headers.get("content-type") ?? "";
  if (!contentType.includes("application/json")) {
    return null;
  }

  return response.json();
}

function handleAuthRedirectResponse(response: Response): never {
  browserLocation.assign(response.url);
  throw new Error("Сессия истекла. Выполняется переход на страницу входа.");
}

function parseFieldErrors(payload: unknown): FieldErrors {
  if (!Array.isArray(payload)) {
    return {};
  }

  const result: FieldErrors = {};
  for (const value of payload) {
    if (!isRecord(value)) {
      continue;
    }

    const field = readOptionalString(value, "field");
    const message = readOptionalString(value, "message");
    if (!field || !message) {
      continue;
    }

    result[field] = [...(result[field] ?? []), message];
  }

  return result;
}

function parseMutationPayload(payload: unknown) {
  if (!isRecord(payload)) {
    return {
      message: "Произошла ошибка при обработке запроса.",
      fieldErrors: {} as FieldErrors,
    };
  }

  return {
    message:
      readOptionalString(payload, "message") ??
      "Произошла ошибка при обработке запроса.",
    fieldErrors: parseFieldErrors(payload.field_errors),
  };
}

async function performMutation(
  url: string,
  init: RequestInit,
): Promise<MutationResult> {
  const response = await fetch(url, {
    ...init,
    credentials: "include",
    headers: {
      Accept: "application/json",
      ...(init.headers ?? {}),
    },
  });

  if (response.redirected) {
    const contentType = response.headers.get("content-type") ?? "";
    if (!contentType.includes("application/json")) {
      handleAuthRedirectResponse(response);
    }
  }

  const payload = parseMutationPayload(await readResponseJson(response));

  if (response.ok) {
    return {
      ok: true,
      message: payload.message,
    };
  }

  return {
    ok: false,
    message: payload.message,
    fieldErrors: payload.fieldErrors,
    status: response.status,
  };
}

function parseShellData(payload: unknown): FilesShellData {
  if (!isRecord(payload) || !isRecord(payload.current_user)) {
    throw new Error("Invalid shell payload.");
  }

  return {
    currentUser: {
      email: readString(payload.current_user, "email"),
      name: readString(payload.current_user, "name"),
      hubId: readNumber(payload.current_user, "hub_id"),
    },
    homeUrl: readString(payload, "home_url"),
  };
}

function parseBrowserEntry(entry: unknown, baseUrl: string): FileBrowserEntry {
  if (!isRecord(entry)) {
    throw new Error("Invalid file-browser entry payload.");
  }

  return {
    name: readString(entry, "name"),
    isDirectory: readBoolean(entry, "is_directory"),
    isImage: readBoolean(entry, "is_image"),
    relativePath: readString(entry, "relative_path"),
    navigationPath: readOptionalString(entry, "navigation_path"),
    downloadUrl: readOptionalString(entry, "download_url")
      ? withBaseUrl(baseUrl, readString(entry, "download_url"))
      : undefined,
    copyUrl: readOptionalString(entry, "copy_url")
      ? withBaseUrl(baseUrl, readString(entry, "copy_url"))
      : undefined,
    previewUrl: readOptionalString(entry, "preview_url")
      ? withBaseUrl(baseUrl, readString(entry, "preview_url"))
      : undefined,
  };
}

function parseBrowserData(
  payload: unknown,
  baseUrl: string,
): FileBrowserApiResponse {
  if (!isRecord(payload) || !Array.isArray(payload.entries)) {
    throw new Error("Invalid file-browser payload.");
  }

  return {
    hubId: readNumber(payload, "hub_id"),
    currentPath: readString(payload, "current_path"),
    entries: payload.entries.map((entry) => parseBrowserEntry(entry, baseUrl)),
  };
}

function parseMenuItems(payload: unknown): UserMenuItem[] {
  if (!Array.isArray(payload)) {
    throw new Error("Invalid menu payload.");
  }

  return payload.map((item) => {
    if (!isRecord(item)) {
      throw new Error("Invalid menu item payload.");
    }

    return {
      name: readString(item, "name"),
      url: readString(item, "url"),
    };
  });
}

export async function fetchShellData(baseUrl: string): Promise<FilesShellData> {
  const payload = await fetchJson(buildIamApiUrl(baseUrl));
  return parseShellData(payload);
}

export async function fetchFileBrowserData(
  baseUrl: string,
  path: string,
): Promise<FileBrowserApiResponse> {
  const payload = await fetchJson(buildEntriesApiUrl(baseUrl, path));
  return parseBrowserData(payload, baseUrl);
}

export async function fetchHubMenuItems(
  authBaseUrl: string,
  hubId: number,
): Promise<UserMenuItem[]> {
  const payload = await fetchJson(
    withBaseUrl(authBaseUrl, `/api/v1/hubs/${hubId}/menu-items`),
  );
  return parseMenuItems(payload);
}

export async function bootstrapFilesPage(
  baseUrl: string,
): Promise<FilesPageBootstrapData> {
  const initialPath =
    new URLSearchParams(window.location.search).get("path") ?? "";
  const shell = await fetchShellData(baseUrl);
  const browser = await fetchFileBrowserData(baseUrl, initialPath);

  return {
    baseUrl,
    initialPath,
    runtimeOwner: "react-shell",
    sharedBrowserComponent: "FileBrowser",
    shell,
    menu: [],
    browser,
  };
}

export function toViewModel(
  data: FileBrowserApiResponse,
): FileBrowserViewModel {
  return {
    hubId: data.hubId,
    currentPath: data.currentPath,
    entries: data.entries,
  };
}

export async function uploadFile(
  baseUrl: string,
  path: string,
  file: File,
): Promise<MutationResult> {
  const body = new FormData();
  body.append("file", file);

  return performMutation(buildUploadUrl(baseUrl, path), {
    method: "POST",
    body,
  });
}

export async function createFolder(
  baseUrl: string,
  path: string,
  name: string,
): Promise<MutationResult> {
  const body = new URLSearchParams();
  body.set("name", name);

  return performMutation(buildCreateFolderUrl(baseUrl, path), {
    method: "POST",
    body,
    headers: {
      "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
    },
  });
}
