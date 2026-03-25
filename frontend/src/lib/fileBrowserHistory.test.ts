import { describe, expect, it } from "vitest";

import {
  parsePathFromSearch,
  updatePathInUrl,
} from "./fileBrowserHistory";

describe("fileBrowserHistory", () => {
  it("parses path from the query string", () => {
    expect(parsePathFromSearch("?path=folder%2Fnested")).toBe("folder/nested");
    expect(parsePathFromSearch("")).toBe("");
  });

  it("updates URLs with or without path", () => {
    expect(updatePathInUrl("https://example.com/", "nested")).toBe(
      "https://example.com/?path=nested",
    );
    expect(updatePathInUrl("https://example.com/?path=nested", "")).toBe(
      "https://example.com/",
    );
  });
});
