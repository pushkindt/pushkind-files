import { describe, expect, it } from "vitest";

import { buildInfo } from "./buildInfo";

describe("buildInfo", () => {
  it("describes the phase 7 frontend shell", () => {
    expect(buildInfo.phase).toBe("phase-7");
    expect(buildInfo.runtimeOwner).toBe("react-shell");
    expect(buildInfo.sharedBrowserComponent).toBe("FileBrowser");
  });
});
