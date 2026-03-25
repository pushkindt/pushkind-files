import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { registerReactFileBrowserMount } from "./reactFileBrowserMount";
import { FilesPageBootstrap } from "../pages/FilesPageBootstrap";

const ROOT_ELEMENT_ID = "react-root";

export function mountFilesPage() {
  const container = document.getElementById(ROOT_ELEMENT_ID);

  if (!container) {
    throw new Error(
      `Missing #${ROOT_ELEMENT_ID} mount node for the files page frontend.`,
    );
  }

  registerReactFileBrowserMount();

  createRoot(container).render(
    <StrictMode>
      <FilesPageBootstrap />
    </StrictMode>,
  );
}
