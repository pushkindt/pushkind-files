(function () {
    function normalizeBase(baseUrl) {
        if (!baseUrl) return "";
        return baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
    }

    function withBase(baseUrl, path) {
        if (!path) return baseUrl;

        // If `path` is already absolute, don't prepend the base.
        // This prevents cases like: https://images.pushkind.comhttps://images.pushkind.com/upload/...
        if (
            path.startsWith("http://") ||
            path.startsWith("https://") ||
            path.startsWith("//")
        ) {
            return path;
        }

        if (!baseUrl) return path;
        if (path.startsWith("/")) return `${baseUrl}${path}`;
        return `${baseUrl}/${path}`;
    }

    function buildBrowserUrl(baseUrl, path) {
        if (path) {
            return withBase(baseUrl, `/files/browser?path=${encodeURIComponent(path)}`);
        }
        return withBase(baseUrl, "/files/browser");
    }

    function buildUploadUrl(baseUrl, path) {
        if (path) {
            return withBase(baseUrl, `/files/upload?path=${encodeURIComponent(path)}`);
        }
        return withBase(baseUrl, "/files/upload");
    }

    function buildCreateFolderUrl(baseUrl, path) {
        if (path) {
            return withBase(baseUrl, `/folder/create?path=${encodeURIComponent(path)}`);
        }
        return withBase(baseUrl, "/folder/create");
    }

    function getPathFromLocation() {
        const url = new URL(window.location.href);
        return url.searchParams.get("path") || "";
    }

    function updateBrowserHistory(path, replace) {
        const url = new URL(window.location.href);
        if (path) {
            url.searchParams.set("path", path);
        } else {
            url.searchParams.delete("path");
        }
        const state = { path: path || "" };
        if (replace) {
            window.history.replaceState(state, "", url.toString());
        } else {
            window.history.pushState(state, "", url.toString());
        }
    }

    function mountFileBrowser(targetSelector, initialPath, options) {
        const host =
            typeof targetSelector === "string"
                ? document.querySelector(targetSelector)
                : targetSelector;

        if (!host) {
            console.warn("[filebrowser] host element not found", targetSelector);
            return;
        }

        const opts = options || {};
        const baseUrl = normalizeBase(opts.baseUrl || "");
        let currentPath = initialPath || "";
        const shouldManageHistory = baseUrl === "";

        async function loadBrowser(path, options = {}) {
            currentPath = path || "";
            const { updateHistory = false, replaceHistory = false } = options;

            if (shouldManageHistory && updateHistory) {
                updateBrowserHistory(currentPath, replaceHistory);
            }

            const res = await fetch(buildBrowserUrl(baseUrl, currentPath), {
                credentials: "include",
            });
            if (!res.ok) {
                throw new Error(`Failed to load file browser: ${res.statusText}`);
            }
            const html = await res.text();
            host.innerHTML = html;
            wireCurrentDom();
        }

        function wireUpload(dropzone, fileInput, uploadList) {
            if (!dropzone || !fileInput || !uploadList) return;

            function handleFiles(files) {
                if (!files || files.length === 0) return;

                uploadList.innerHTML = "";
                let completed = 0;
                dropzone.classList.add("d-none");
                fileInput.disabled = true;

                Array.from(files).forEach((file) => {
                    const formData = new FormData();
                    formData.append("file", file);

                    const id = "upload_" + Math.random().toString(36).slice(2, 10);
                    uploadList.insertAdjacentHTML(
                        "beforeend",
                        `
                        <div class="mb-2" id="${id}_wrapper">
                            <div class="small mb-1">Uploading: ${file.name}</div>
                            <div class="spinner-border spinner-border-sm text-primary" role="status"></div>
                        </div>
                    `,
                    );

                    fetch(buildUploadUrl(baseUrl, currentPath), {
                        method: "POST",
                        body: formData,
                        credentials: "include",
                    })
                        .then((res) => {
                            if (!res.ok) throw new Error(`Не удалось загрузить ${file.name}.`);
                            return res.text();
                        })
                        .then(() => {
                            const wrapper = document.getElementById(`${id}_wrapper`);
                            if (wrapper) {
                                wrapper.innerHTML = `✅ Загружен: <strong>${file.name}</strong>`;
                            }
                        })
                        .catch((err) => {
                            const wrapper = document.getElementById(`${id}_wrapper`);
                            if (wrapper) {
                                wrapper.innerHTML = `❌ Ошибка загрузки <strong>${file.name}</strong>: ${err.message}`;
                            }
                        })
                        .finally(() => {
                            completed += 1;
                            if (completed === files.length) {
                                setTimeout(() => {
                                    loadBrowser(currentPath).catch((err) =>
                                        console.error("[filebrowser] reload failed", err),
                                    );
                                }, 800);
                            }
                        });
                });
            }

            dropzone.addEventListener("dragover", (e) => {
                e.preventDefault();
                dropzone.classList.add("dragover");
            });

            dropzone.addEventListener("dragleave", () => {
                dropzone.classList.remove("dragover");
            });

            dropzone.addEventListener("drop", (e) => {
                e.preventDefault();
                dropzone.classList.remove("dragover");
                handleFiles(e.dataTransfer.files);
            });

            fileInput.addEventListener("change", (e) => {
                handleFiles(e.target.files);
            });
        }

        function wireFolderForm(form, panel, hidePanel) {
            if (!form || form.dataset.wired === "true") return;
            form.dataset.wired = "true";

            form.addEventListener("submit", (e) => {
                e.preventDefault();

                const submitBtn = form.querySelector('button[type="submit"]');
                if (submitBtn) submitBtn.disabled = true;

                const formData = new URLSearchParams(new FormData(form));
                fetch(buildCreateFolderUrl(baseUrl, currentPath), {
                    method: "POST",
                    body: formData,
                    headers: {
                        "Content-Type": "application/x-www-form-urlencoded",
                    },
                    credentials: "include",
                })
                    .then(async (res) => {
                        if (!res.ok) {
                            const msg = (await res.text()) || "Не удалось создать папку";
                            throw new Error(msg);
                        }
                    })
                    .then(() => {
                        if (hidePanel) hidePanel();
                        form.reset();
                        return loadBrowser(currentPath);
                    })
                    .catch((err) => {
                        console.error(err);
                        alert(err.message || "Ошибка создания папки");
                    })
                    .finally(() => {
                        if (submitBtn) submitBtn.disabled = false;
                        if (panel && !panel.classList.contains("d-none")) {
                            const input = panel.querySelector("input");
                            if (input) input.focus();
                        }
                    });
            });
        }

        function wireCurrentDom() {
            const dropzone = host.querySelector("[data-dropzone]");
            const fileInput = host.querySelector("[data-file-input]");
            const uploadList = host.querySelector("[data-upload-progress]");
            wireUpload(dropzone, fileInput, uploadList);

            const panel = host.querySelector("[data-new-folder-panel]");
            const form = host.querySelector("[data-new-folder-form]");
            const toggleBtn = host.querySelector("[data-new-folder-toggle]");
            const cancelBtns = host.querySelectorAll("[data-new-folder-cancel]");
            const nameInput = host.querySelector("#folderName");

            const showPanel = () => {
                if (!panel) return;
                panel.classList.remove("d-none");
                if (nameInput) {
                    setTimeout(() => nameInput.focus(), 50);
                }
            };

            const hidePanel = () => {
                if (panel && !panel.classList.contains("d-none")) {
                    panel.classList.add("d-none");
                }
            };

            if (toggleBtn && toggleBtn.dataset.wired !== "true") {
                toggleBtn.dataset.wired = "true";
                toggleBtn.addEventListener("click", (e) => {
                    e.preventDefault();
                    if (!panel) return;
                    if (panel.classList.contains("d-none")) {
                        showPanel();
                    } else {
                        hidePanel();
                    }
                });
            }

            cancelBtns.forEach((btn) => {
                if (btn.dataset.wired === "true") return;
                btn.dataset.wired = "true";
                btn.addEventListener("click", (e) => {
                    e.preventDefault();
                    hidePanel();
                });
            });

            wireFolderForm(form, panel, hidePanel);

            // Rewrite file URLs to point at the configured base.
            host.querySelectorAll("[data-file-url]").forEach((el) => {
                const rel = el.dataset.fileUrl;
                if (!rel) return;
                const absolute = withBase(baseUrl, rel);
                if (el.tagName === "IMG") {
                    el.src = absolute;
                } else if (el.tagName === "A") {
                    el.href = absolute;
                }
            });
        }

        host.addEventListener("click", (event) => {
            const nav = event.target.closest(".filebrowser-nav");
            if (nav && nav.dataset.filebrowserTarget !== undefined) {
                event.preventDefault();
                loadBrowser(nav.dataset.filebrowserTarget || "", {
                    updateHistory: true,
                }).catch((err) => console.error("[filebrowser] navigation failed", err));
                return;
            }

            const copyBtn = event.target.closest(".copy-btn");
            if (copyBtn) {
                event.preventDefault();
                event.stopPropagation();
                const url = copyBtn.dataset.fileUrl;
                if (!url) return;

                const fullUrl = withBase(baseUrl || location.origin, url);

                const fallbackCopy = (text) => {
                    const textarea = document.createElement("textarea");
                    textarea.value = text;
                    textarea.setAttribute("readonly", "");
                    textarea.style.position = "absolute";
                    textarea.style.left = "-9999px";
                    document.body.appendChild(textarea);
                    const selection = document.getSelection();
                    const currentRange = selection && selection.rangeCount > 0 ? selection.getRangeAt(0) : null;
                    textarea.select();
                    document.execCommand("copy");
                    document.body.removeChild(textarea);
                    if (selection && currentRange) {
                        selection.removeAllRanges();
                        selection.addRange(currentRange);
                    }
                };

                Promise.resolve()
                    .then(() => {
                        if (navigator.clipboard?.writeText) {
                            return navigator.clipboard.writeText(fullUrl);
                        }
                        fallbackCopy(fullUrl);
                    })
                    .then(() => {
                        copyBtn.innerHTML =
                            '<i class="bi bi-clipboard-check text-success"></i>';
                        setTimeout(() => {
                            copyBtn.innerHTML = '<i class="bi bi-clipboard"></i>';
                        }, 1500);
                    })
                    .catch((err) => {
                        console.error("Copy failed:", err);
                        alert("Failed to copy link");
                    });
            }
        });

        if (shouldManageHistory) {
            const urlPath = getPathFromLocation();
            if (urlPath && urlPath !== currentPath) {
                currentPath = urlPath;
            }
            updateBrowserHistory(currentPath, true);
            window.addEventListener("popstate", (event) => {
                const nextPath =
                    (event.state && typeof event.state.path === "string"
                        ? event.state.path
                        : getPathFromLocation()) || "";
                if (nextPath === currentPath) return;
                loadBrowser(nextPath).catch((err) =>
                    console.error("[filebrowser] popstate navigation failed", err),
                );
            });
        }

        loadBrowser(currentPath).catch((err) => {
            console.error("[filebrowser] initial load failed", err);
            host.innerHTML =
                '<div class="alert alert-danger">Не удалось загрузить файловый браузер.</div>';
        });

        return {
            reload: () => loadBrowser(currentPath),
            navigate: (path) => loadBrowser(path),
            getPath: () => currentPath,
        };
    }

    window.mountFileBrowser = mountFileBrowser;
})();
