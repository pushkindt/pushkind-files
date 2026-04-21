import type { ApiMutationError } from "@pushkind/frontend-shell/mutations";
import {
  isApiMutationError,
  postForm,
  postMultipartForm,
} from "@pushkind/frontend-shell/mutations";
import {
  ensureResponseIsNotAuthRedirect,
  fetchHubMenuItems as fetchSharedHubMenuItems,
  fetchNoAccessData as fetchSharedNoAccessData,
  fetchShellData as fetchSharedShellData,
  readJsonResponse,
} from "@pushkind/frontend-shell/shellApi";
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
    headers: {
      Accept: "application/json",
    },
    cache: "no-store",
    credentials: "include",
  });

  ensureResponseIsNotAuthRedirect(response);

  if (!response.ok) {
    if (response.status === 400) {
      throw new Error("Недопустимый путь.");
    }
    if (response.status === 401) {
      throw new Error("Недостаточно прав для доступа к файлам.");
    }
    throw new Error(`Request failed with status ${response.status}.`);
  }

  return readJsonResponse(response, url);
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

async function performMutation(
  request: () => Promise<{ message: string }>,
): Promise<MutationResult> {
  try {
    const payload = await request();
    return {
      ok: true,
      message: payload.message,
    };
  } catch (error) {
    const payload: ApiMutationError = isApiMutationError(error)
      ? error
      : {
          message:
            error instanceof Error
              ? error.message
              : "Произошла ошибка при обработке запроса.",
          field_errors: [],
        };

    return {
      ok: false,
      message: payload.message,
      fieldErrors: parseFieldErrors(payload.field_errors),
    };
  }
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

export async function fetchShellData(baseUrl: string): Promise<FilesShellData> {
  return fetchSharedShellData<FilesShellData>(
    buildIamApiUrl(baseUrl),
    "Недостаточно прав для доступа к файлам.",
  );
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
  return fetchSharedHubMenuItems<UserMenuItem>(
    withBaseUrl(authBaseUrl, `/api/v1/hubs/${hubId}/menu-items`),
    "Не удалось загрузить меню пользователя.",
  );
}

export async function fetchNoAccessData(baseUrl: string) {
  return fetchSharedNoAccessData(
    withBaseUrl(baseUrl, "/api/v1/no-access"),
    "Не удалось загрузить страницу доступа.",
  );
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

  return performMutation(() =>
    postMultipartForm(buildUploadUrl(baseUrl, path), body),
  );
}

export async function createFolder(
  baseUrl: string,
  path: string,
  name: string,
): Promise<MutationResult> {
  const body = new URLSearchParams();
  body.set("name", name);

  return performMutation(() =>
    postForm(buildCreateFolderUrl(baseUrl, path), body),
  );
}
