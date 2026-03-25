import { describe, expect, it } from "vitest";

import {
  buildBrowserUrl,
  buildCreateFolderUrl,
  buildFileDownloadUrl,
  buildUploadUrl,
  normalizeBaseUrl,
  withBaseUrl,
} from "./fileBrowserUrls";

describe("fileBrowserUrls", () => {
  it("normalizes empty and trailing-slash bases", () => {
    expect(normalizeBaseUrl("")).toBe("");
    expect(normalizeBaseUrl("https://example.com/root/")).toBe(
      "https://example.com/root",
    );
  });

  it("builds browser and mutation URLs relative to the base", () => {
    expect(buildBrowserUrl("https://example.com/root", "nested/folder")).toBe(
      "https://example.com/root/files/browser?path=nested%2Ffolder",
    );
    expect(buildUploadUrl("", "nested/folder")).toBe(
      "/files/upload?path=nested%2Ffolder",
    );
    expect(buildCreateFolderUrl("", "")).toBe("/folder/create");
  });

  it("builds download URLs and preserves absolute inputs", () => {
    expect(buildFileDownloadUrl("", 7, "poster.png")).toBe(
      "/upload/7/poster.png",
    );
    expect(
      withBaseUrl("https://example.com/root", "https://cdn.example.com/file"),
    ).toBe("https://cdn.example.com/file");
  });
});
