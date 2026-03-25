import { act } from "react";
import { createRoot } from "react-dom/client";
import { afterEach, describe, expect, it } from "vitest";

import { FileBrowser } from "./FileBrowser";
import { sampleBrowserModel } from "../lib/fileBrowserFixtures";

type Deferred<T> = {
  promise: Promise<T>;
  resolve: (value: T) => void;
};

function deferred<T>(): Deferred<T> {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((innerResolve) => {
    resolve = innerResolve;
  });

  return { promise, resolve };
}

let cleanup: (() => void) | null = null;

afterEach(() => {
  cleanup?.();
  cleanup = null;
});

describe("FileBrowser", () => {
  it("re-enables folder submission after a successful create", async () => {
    const container = document.createElement("div");
    document.body.appendChild(container);
    const root = createRoot(container);
    const request = deferred<{ ok: true; message: string }>();

    cleanup = () => {
      root.unmount();
      container.remove();
    };

    await act(async () => {
      root.render(
        <FileBrowser
          model={sampleBrowserModel}
          baseUrl=""
          historyMode="managed"
          onCreateFolder={() => request.promise}
        />,
      );
    });

    const toggleButton = container.querySelector("[data-new-folder-toggle]");
    expect(toggleButton).toBeInstanceOf(HTMLButtonElement);

    await act(async () => {
      (toggleButton as HTMLButtonElement).click();
    });

    const input = container.querySelector('input[name="name"]');
    expect(input).toBeInstanceOf(HTMLInputElement);

    await act(async () => {
      (input as HTMLInputElement).value = "Gallery";
      input?.dispatchEvent(new Event("input", { bubbles: true }));
    });

    const form = container.querySelector("[data-new-folder-form]");
    expect(form).toBeInstanceOf(HTMLFormElement);

    await act(async () => {
      form?.dispatchEvent(
        new Event("submit", { bubbles: true, cancelable: true }),
      );
    });

    let submitButton = container.querySelector(
      '[data-new-folder-form] button[type="submit"]',
    );
    expect(submitButton).toBeInstanceOf(HTMLButtonElement);
    expect((submitButton as HTMLButtonElement).disabled).toBe(true);

    await act(async () => {
      request.resolve({ ok: true, message: "Created." });
      await request.promise;
    });

    await act(async () => {
      (toggleButton as HTMLButtonElement).click();
    });

    submitButton = container.querySelector(
      '[data-new-folder-form] button[type="submit"]',
    );
    expect(submitButton).toBeInstanceOf(HTMLButtonElement);
    expect((submitButton as HTMLButtonElement).disabled).toBe(false);
    expect((submitButton as HTMLButtonElement).textContent).toContain(
      "Создать",
    );
  });
});
