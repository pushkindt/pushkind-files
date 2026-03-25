(function () {
  function normalizeBaseUrl(baseUrl) {
    if (!baseUrl) {
      return "";
    }

    return baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
  }

  function withBaseUrl(baseUrl, path) {
    if (!path) {
      return normalizeBaseUrl(baseUrl);
    }

    if (
      path.startsWith("http://") ||
      path.startsWith("https://") ||
      path.startsWith("//")
    ) {
      return path;
    }

    const normalizedBase = normalizeBaseUrl(baseUrl);
    if (!normalizedBase) {
      return path;
    }

    if (path.startsWith("/")) {
      return normalizedBase + path;
    }

    return normalizedBase + "/" + path;
  }

  function defaultBaseUrl() {
    const currentScript = document.currentScript;
    if (!(currentScript instanceof HTMLScriptElement) || !currentScript.src) {
      return "";
    }

    return new URL(currentScript.src, window.location.href).origin;
  }

  function resolveTarget(target) {
    if (typeof target === "string") {
      const element = document.querySelector(target);
      if (!element) {
        throw new Error("File browser mount target not found: " + target);
      }

      return element;
    }

    return target;
  }

  function buildBrowserDocumentUrl(baseUrl, path) {
    const documentUrl = withBaseUrl(baseUrl, "/assets/dist/app/browser.html");
    const url = new URL(documentUrl, window.location.href);

    if (path) {
      url.searchParams.set("path", path);
    }

    return url.toString();
  }

  function createFrame(src) {
    const iframe = document.createElement("iframe");
    iframe.src = src;
    iframe.loading = "lazy";
    iframe.referrerPolicy = "strict-origin-when-cross-origin";
    iframe.style.width = "100%";
    iframe.style.height = "18rem";
    iframe.style.border = "0";
    iframe.style.display = "block";
    iframe.style.overflow = "hidden";
    iframe.setAttribute("title", "File browser");
    iframe.setAttribute("scrolling", "no");
    return iframe;
  }

  function mountFileBrowser(target, initialPath, options) {
    const host = resolveTarget(target);
    const baseUrl = normalizeBaseUrl((options && options.baseUrl) || defaultBaseUrl());
    let currentPath = initialPath || "";
    let frame = createFrame(buildBrowserDocumentUrl(baseUrl, currentPath));
    let disposed = false;

    function handleMessage(event) {
      if (disposed || event.source !== frame.contentWindow) {
        return;
      }

      const data = event.data;
      if (!data || data.source !== "pushkind-files" || data.type !== "embedded-file-browser:resize") {
        return;
      }

      const nextHeight = Number(data.height);
      if (!Number.isFinite(nextHeight) || nextHeight <= 0) {
        return;
      }

      frame.style.height = Math.max(220, Math.ceil(nextHeight)) + "px";
    }

    function replaceFrame(nextPath) {
      currentPath = nextPath || "";
      const nextFrame = createFrame(buildBrowserDocumentUrl(baseUrl, currentPath));
      host.replaceChildren(nextFrame);
      frame = nextFrame;
    }

    window.addEventListener("message", handleMessage);
    host.replaceChildren(frame);

    return {
      dispose() {
        disposed = true;
        window.removeEventListener("message", handleMessage);
        if (frame && frame.parentNode === host) {
          host.removeChild(frame);
        }
      },
      async navigate(path) {
        replaceFrame(path);
      },
      async reload() {
        replaceFrame(currentPath);
      },
      getPath() {
        return currentPath;
      },
    };
  }

  window.mountFileBrowser = mountFileBrowser;
})();
