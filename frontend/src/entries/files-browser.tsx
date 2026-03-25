import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { EmbeddedBrowserBootstrap } from "../pages/EmbeddedBrowserBootstrap";
import { registerReactFileBrowserMount } from "../lib/reactFileBrowserMount";

const ROOT_ELEMENT_ID = "react-root";

registerReactFileBrowserMount();

const container = document.getElementById(ROOT_ELEMENT_ID);

if (!container) {
  throw new Error(
    `Missing #${ROOT_ELEMENT_ID} mount node for the embedded browser frontend.`,
  );
}

createRoot(container).render(
  <StrictMode>
    <EmbeddedBrowserBootstrap />
  </StrictMode>,
);
