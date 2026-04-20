import { afterEach, describe, expect, it, vi } from "vitest";

import { createFolder } from "./filesApi";

describe("filesApi mutations", () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("returns structured failure data for unauthorized JSON responses", async () => {
    const fetchMock = vi.spyOn(globalThis, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          message: "Сессия истекла. Войдите снова и повторите действие.",
          field_errors: [],
        }),
        {
          status: 401,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    const result = await createFolder("", "", "docs");

    expect(fetchMock).toHaveBeenCalledOnce();
    expect(result).toEqual({
      ok: false,
      message: "Сессия истекла. Войдите снова и повторите действие.",
      fieldErrors: {},
    });
  });
});
