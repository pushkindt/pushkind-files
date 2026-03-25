import { EmbeddedFileBrowser } from "../components/EmbeddedFileBrowser";
import { isFixtureMode } from "../lib/fixtureMode";
import { sampleBrowserApiResponse } from "../lib/fileBrowserFixtures";
import "../styles/files-page.css";

export function EmbeddedBrowserBootstrap() {
  const initialPath =
    new URLSearchParams(window.location.search).get("path") ?? "";
  const fixtureData = isFixtureMode() ? sampleBrowserApiResponse : undefined;

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
