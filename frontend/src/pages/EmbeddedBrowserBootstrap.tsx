import { useEffect } from "react";

import { EmbeddedFileBrowser } from "../components/EmbeddedFileBrowser";
import { isFixtureMode } from "../lib/fixtureMode";
import { sampleBrowserApiResponse } from "../lib/fileBrowserFixtures";
import "../styles/files-page.css";

export function EmbeddedBrowserBootstrap() {
  const initialPath =
    new URLSearchParams(window.location.search).get("path") ?? "";
  const fixtureData = isFixtureMode() ? sampleBrowserApiResponse : undefined;

  useEffect(() => {
    const postHeight = () => {
      const height = Math.ceil(
        Math.max(
          document.body?.scrollHeight ?? 0,
          document.documentElement?.scrollHeight ?? 0,
        ),
      );

      window.parent.postMessage(
        {
          source: "pushkind-files",
          type: "embedded-file-browser:resize",
          height,
        },
        "*",
      );
    };

    const resizeObserver = new ResizeObserver(() => {
      postHeight();
    });

    if (document.body) {
      resizeObserver.observe(document.body);
    }

    if (document.documentElement) {
      resizeObserver.observe(document.documentElement);
    }

    window.addEventListener("load", postHeight);
    window.addEventListener("resize", postHeight);
    postHeight();

    return () => {
      resizeObserver.disconnect();
      window.removeEventListener("load", postHeight);
      window.removeEventListener("resize", postHeight);
    };
  }, []);

  return (
    <main className="py-3">
      <EmbeddedFileBrowser
        initialPath={initialPath}
        options={{ baseUrl: "", historyMode: "disabled" }}
        initialData={fixtureData}
      />
    </main>
  );
}
