import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { FileBrowser } from "../components/FileBrowser";
import { sampleBrowserModel } from "./fileBrowserFixtures";

describe("FileBrowser", () => {
  it("renders a typed browser scaffold without Tera markup", () => {
    const html = renderToStaticMarkup(
      <FileBrowser
        model={sampleBrowserModel}
        baseUrl=""
        historyMode="managed"
      />,
    );

    expect(html).toContain("Новая папка");
    expect(html).toContain("Фото");
    expect(html).toContain("poster.png");
    expect(html).toContain(
      "/?path=%D0%91%D0%B8%D0%B1%D0%BB%D0%B8%D0%BE%D1%82%D0%B5%D0%BA%D0%B0%2F%D0%A4%D0%BE%D1%82%D0%BE",
    );
    expect(html).toContain(
      "/files/upload?path=%D0%91%D0%B8%D0%B1%D0%BB%D0%B8%D0%BE%D1%82%D0%B5%D0%BA%D0%B0",
    );
  });
});
