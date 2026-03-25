(function () {
  const DIST_ROOT = "/assets/dist";
  const MANIFEST_PATH = DIST_ROOT + "/manifest.json";
  const ENTRY_KEY = "app/browser.html";

  const loaderState = {
    pending: null,
    styleHrefs: new Set(),
    scriptSrcs: new Set(),
  };

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

  function collectCss(manifest, key, acc) {
    const chunk = manifest[key];
    if (!chunk) {
      return;
    }

    if (Array.isArray(chunk.css)) {
      chunk.css.forEach((href) => acc.add(href));
    }

    if (Array.isArray(chunk.imports)) {
      chunk.imports.forEach((importKey) => collectCss(manifest, importKey, acc));
    }
  }

  function ensureStyle(baseUrl, href) {
    const absoluteHref = withBaseUrl(baseUrl, DIST_ROOT + "/" + href);
    if (loaderState.styleHrefs.has(absoluteHref)) {
      return;
    }

    loaderState.styleHrefs.add(absoluteHref);

    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = absoluteHref;
    link.crossOrigin = "";
    document.head.appendChild(link);
  }

  function loadModuleScript(baseUrl, src) {
    const absoluteSrc = withBaseUrl(baseUrl, DIST_ROOT + "/" + src);
    if (loaderState.scriptSrcs.has(absoluteSrc)) {
      return Promise.resolve();
    }

    loaderState.scriptSrcs.add(absoluteSrc);

    return new Promise((resolve, reject) => {
      const script = document.createElement("script");
      script.type = "module";
      script.src = absoluteSrc;
      script.crossOrigin = "";
      script.onload = () => resolve();
      script.onerror = () => reject(new Error("Failed to load embedded file browser runtime."));
      document.head.appendChild(script);
    });
  }

  async function loadRuntime(baseUrl) {
    const manifestResponse = await fetch(withBaseUrl(baseUrl, MANIFEST_PATH), {
      credentials: "include",
    });
    if (!manifestResponse.ok) {
      throw new Error("Failed to load embedded file browser manifest.");
    }

    const manifest = await manifestResponse.json();
    const entry = manifest[ENTRY_KEY];
    if (!entry || typeof entry.file !== "string") {
      throw new Error("Embedded file browser entry is missing from the manifest.");
    }

    const cssFiles = new Set();
    collectCss(manifest, ENTRY_KEY, cssFiles);
    cssFiles.forEach((href) => ensureStyle(baseUrl, href));

    await loadModuleScript(baseUrl, entry.file);
  }

  async function ensureRuntime(baseUrl) {
    if (
      typeof window.mountFileBrowser === "function" &&
      window.mountFileBrowser !== compatibilityMountFileBrowser
    ) {
      return;
    }

    if (!loaderState.pending) {
      loaderState.pending = loadRuntime(baseUrl).finally(() => {
        loaderState.pending = null;
      });
    }

    await loaderState.pending;

    if (
      typeof window.mountFileBrowser !== "function" ||
      window.mountFileBrowser === compatibilityMountFileBrowser
    ) {
      throw new Error("Embedded file browser runtime did not register mountFileBrowser.");
    }
  }

  function compatibilityMountFileBrowser(target, initialPath, options) {
    const baseUrl = normalizeBaseUrl((options && options.baseUrl) || defaultBaseUrl());
    let mounted = null;

    const ready = ensureRuntime(baseUrl).then(() => {
      mounted = window.mountFileBrowser(target, initialPath, Object.assign({}, options, { baseUrl }));
      return mounted;
    });

    return {
      dispose() {
        void ready.then((instance) => instance.dispose());
      },
      async navigate(path) {
        const instance = await ready;
        return instance.navigate(path);
      },
      async reload() {
        const instance = await ready;
        return instance.reload();
      },
      getPath() {
        return mounted ? mounted.getPath() : initialPath;
      },
    };
  }

  window.mountFileBrowser = compatibilityMountFileBrowser;
})();
