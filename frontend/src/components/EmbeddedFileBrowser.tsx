import { useEffect, useState } from "react";

import {
  createFolder,
  fetchFileBrowserData,
  toViewModel,
  uploadFile,
} from "../lib/filesApi";
import type {
  FileBrowserApiResponse,
  MountFileBrowserOptions,
} from "../lib/fileBrowserModels";
import { resolveMountOptions } from "../lib/fileBrowserUrls";
import { FileBrowser } from "./FileBrowser";

type EmbeddedFileBrowserProps = {
  initialPath: string;
  options?: MountFileBrowserOptions;
  initialData?: FileBrowserApiResponse;
  onPathChange?: (path: string) => void;
};

export function EmbeddedFileBrowser({
  initialPath,
  options,
  initialData,
  onPathChange,
}: EmbeddedFileBrowserProps) {
  const { baseUrl, historyMode } = resolveMountOptions(options);
  const [data, setData] = useState<FileBrowserApiResponse | null>(
    initialData ?? null,
  );
  const [error, setError] = useState<string | null>(null);

  async function load(path: string) {
    try {
      const nextData = await fetchFileBrowserData(baseUrl, path);
      setData(nextData);
      setError(null);
      onPathChange?.(nextData.currentPath);
      return nextData;
    } catch (loadError) {
      setError(
        loadError instanceof Error
          ? loadError.message
          : "Не удалось загрузить файлы.",
      );
      return null;
    }
  }

  async function refreshCurrentDirectory() {
    if (!data) {
      return;
    }

    await load(data.currentPath);
  }

  useEffect(() => {
    if (initialData) {
      onPathChange?.(initialData.currentPath);
      return;
    }

    void load(initialPath);
  }, [initialData, initialPath]);

  if (error) {
    return <div className="alert alert-danger">{error}</div>;
  }

  if (!data) {
    return <div className="alert alert-secondary">Загрузка файлов...</div>;
  }

  return (
    <FileBrowser
      model={toViewModel(data)}
      baseUrl={baseUrl}
      historyMode={historyMode}
      onNavigate={(path) => {
        void load(path);
      }}
      onUploadFile={async (file) => {
        const result = await uploadFile(baseUrl, data.currentPath, file);
        if (result.ok) {
          await refreshCurrentDirectory();
        }
        return result;
      }}
      onCreateFolder={async (name) => {
        const result = await createFolder(baseUrl, data.currentPath, name);
        if (result.ok) {
          await refreshCurrentDirectory();
        }
        return result;
      }}
    />
  );
}
