import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { NoAccessPage } from "../pages/NoAccessPage";
import "../styles/files-page.css";

const ROOT_ELEMENT_ID = "react-root";

const container = document.getElementById(ROOT_ELEMENT_ID);

if (!container) {
  throw new Error(
    `Missing #${ROOT_ELEMENT_ID} mount node for the no-access frontend.`,
  );
}

createRoot(container).render(
  <StrictMode>
    <NoAccessPage />
  </StrictMode>,
);
