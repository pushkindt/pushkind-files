import { afterEach, describe, expect, it, vi } from "vitest";

import { browserLocation, createFolder } from "./filesApi";

describe("filesApi auth redirect handling", () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("navigates to the redirected auth page before mutation fallback parsing", async () => {
    const fetchMock = vi.spyOn(globalThis, "fetch").mockResolvedValue({
      redirected: true,
      url: "https://users.pushkind.com/auth/signin?next=%2F",
      status: 200,
      ok: true,
      headers: new Headers({ "content-type": "text/html; charset=utf-8" }),
      json: vi.fn(),
    } as unknown as Response);
    const assignSpy = vi
      .spyOn(browserLocation, "assign")
      .mockImplementation(() => undefined);

    await expect(createFolder("", "", "docs")).rejects.toThrow(
      "Сессия истекла.",
    );

    expect(fetchMock).toHaveBeenCalledOnce();
    expect(assignSpy).toHaveBeenCalledWith(
      "https://users.pushkind.com/auth/signin?next=%2F",
    );
  });
});
