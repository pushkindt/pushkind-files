import { useEffect, useState } from "react";

import { FileBrowser } from "../components/FileBrowser";
import { FilesAppShell } from "../components/FilesAppShell";
import { FilesPageFatalState } from "../components/FilesPageFatalState";
import { FilesPageLoadingState } from "../components/FilesPageLoadingState";
import { FlashStack } from "../components/FlashStack";
import { UserMenu } from "../components/UserMenu";
import { bootstrapFilesPage } from "../lib/bootstrapFilesPage";
import { isFixtureMode } from "../lib/fixtureMode";
import {
  sampleBrowserApiResponse,
  sampleMenuItems,
  sampleShellData,
} from "../lib/fileBrowserFixtures";
import {
  createFolder,
  fetchFileBrowserData,
  toViewModel,
  uploadFile,
} from "../lib/filesApi";
import {
  getPathFromLocation,
  syncBrowserHistory,
} from "../lib/fileBrowserHistory";
import type { FilesPageBootstrapData } from "../lib/fileBrowserModels";
import "../styles/files-page.css";

type FilesPageState =
  | { status: "loading" }
  | { status: "ready"; data: FilesPageBootstrapData }
  | { status: "error"; message: string };

export function FilesPageBootstrap() {
  const [state, setState] = useState<FilesPageState>({ status: "loading" });

  useEffect(() => {
    let active = true;

    async function bootstrap() {
      try {
        const data = isFixtureMode()
          ? {
              baseUrl: "",
              initialPath: sampleBrowserApiResponse.currentPath,
              runtimeOwner: "react-shell" as const,
              sharedBrowserComponent: "FileBrowser" as const,
              shell: sampleShellData,
              menu: sampleMenuItems,
              browser: sampleBrowserApiResponse,
            }
          : await bootstrapFilesPage("");

        if (!active) {
          return;
        }

        setState({ status: "ready", data });
      } catch (error) {
        if (!active) {
          return;
        }

        setState({
          status: "error",
          message:
            error instanceof Error
              ? error.message
              : "Не удалось инициализировать страницу файлов.",
        });
      }
    }

    void bootstrap();

    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    if (state.status !== "ready") {
      return;
    }

    const onPopState = () => {
      void navigateToPath(getPathFromLocation(window.location), {
        fromPopState: true,
      });
    };

    syncBrowserHistory(state.data.browser.currentPath, true);
    window.addEventListener("popstate", onPopState);

    return () => {
      window.removeEventListener("popstate", onPopState);
    };
  }, [state]);

  async function loadDirectory(path: string) {
    if (state.status !== "ready") {
      throw new Error("Файловый браузер еще не готов.");
    }

    return fetchFileBrowserData(state.data.baseUrl, path);
  }

  async function navigateToPath(
    path: string,
    options?: { replace?: boolean; fromPopState?: boolean },
  ) {
    if (state.status !== "ready") {
      return;
    }

    try {
      const browser = await loadDirectory(path);
      setState((currentState) => {
        if (currentState.status !== "ready") {
          return currentState;
        }

        return {
          status: "ready",
          data: {
            ...currentState.data,
            browser,
          },
        };
      });

      if (!options?.fromPopState) {
        syncBrowserHistory(path, options?.replace ?? false);
      }
    } catch (error) {
      setState({
        status: "error",
        message:
          error instanceof Error
            ? error.message
            : "Не удалось загрузить файлы.",
      });
    }
  }

  async function refreshCurrentDirectory() {
    if (state.status !== "ready") {
      return;
    }

    const browser = await loadDirectory(state.data.browser.currentPath);
    setState((currentState) => {
      if (currentState.status !== "ready") {
        return currentState;
      }

      return {
        status: "ready",
        data: {
          ...currentState.data,
          browser,
        },
      };
    });
  }

  if (state.status === "loading") {
    return <FilesPageLoadingState />;
  }

  if (state.status === "error") {
    return <FilesPageFatalState message={state.message} />;
  }

  return (
    <FilesAppShell
      userMenu={<UserMenu shell={state.data.shell} items={state.data.menu} />}
      flashes={<FlashStack />}
    >
      <FileBrowser
        model={toViewModel(state.data.browser)}
        baseUrl={state.data.baseUrl}
        historyMode="managed"
        onNavigate={(path) => {
          void navigateToPath(path);
        }}
        onUploadFile={async (file) => {
          const result = await uploadFile(
            state.data.baseUrl,
            state.data.browser.currentPath,
            file,
          );

          if (result.ok) {
            await refreshCurrentDirectory();
          }

          return result;
        }}
        onCreateFolder={async (name) => {
          const result = await createFolder(
            state.data.baseUrl,
            state.data.browser.currentPath,
            name,
          );

          if (result.ok) {
            await refreshCurrentDirectory();
          }

          return result;
        }}
      />
    </FilesAppShell>
  );
}
