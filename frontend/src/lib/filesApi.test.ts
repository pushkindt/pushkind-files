import { afterEach, describe, expect, it, vi } from "vitest";

import { createFolder, fetchShellData } from "./filesApi";

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

  it("loads shell data from the regular IAM endpoint", async () => {
    const fetchMock = vi.spyOn(globalThis, "fetch").mockResolvedValue(
      new Response(
        JSON.stringify({
          current_user: {
            email: "blocked@example.com",
            name: "Blocked User",
            hub_id: 42,
            roles: [],
          },
          home_url: "https://auth.example.com",
          navigation: [],
          local_menu_items: [],
          hub_name: "Files",
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        },
      ),
    );

    const shell = await fetchShellData("");

    expect(fetchMock).toHaveBeenCalledWith("/api/v1/iam", {
      headers: {
        Accept: "application/json",
      },
      cache: "no-store",
      credentials: "include",
    });
    expect(shell.currentUser.email).toBe("blocked@example.com");
  });
});
