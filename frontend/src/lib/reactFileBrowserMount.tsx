import { createRoot, type Root } from "react-dom/client";

import { EmbeddedFileBrowser } from "../components/EmbeddedFileBrowser";
import type { MountFileBrowserOptions } from "./fileBrowserModels";

type MountTarget = string | Element;

type MountedFileBrowser = {
  dispose: () => void;
  navigate: (path: string) => Promise<void>;
  reload: () => Promise<void>;
  getPath: () => string;
};

declare global {
  interface Window {
    mountFileBrowser?: (
      target: MountTarget,
      initialPath: string,
      options?: MountFileBrowserOptions,
    ) => MountedFileBrowser;
  }
}

function resolveTarget(target: MountTarget) {
  if (typeof target === "string") {
    const element = document.querySelector(target);
    if (!element) {
      throw new Error(`File browser mount target not found: ${target}`);
    }

    return element;
  }

  return target;
}

export function mountFileBrowser(
  target: MountTarget,
  initialPath: string,
  options?: MountFileBrowserOptions,
) {
  const element = resolveTarget(target);
  const root: Root = createRoot(element);
  let currentPath = initialPath;

  const render = (path: string) => {
    root.render(
      <EmbeddedFileBrowser
        initialPath={path}
        options={options}
        onPathChange={(nextPath) => {
          currentPath = nextPath;
        }}
      />,
    );
  };

  render(initialPath);

  return {
    dispose() {
      root.unmount();
    },
    async navigate(path: string) {
      currentPath = path;
      render(path);
    },
    async reload() {
      render(currentPath);
    },
    getPath() {
      return currentPath;
    },
  };
}

export function registerReactFileBrowserMount() {
  window.mountFileBrowser = mountFileBrowser;
}
