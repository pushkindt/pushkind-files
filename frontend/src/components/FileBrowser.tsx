import { useMemo, useRef, useState } from "react";

import {
  buildBrowserUrl,
  buildCreateFolderUrl,
  buildMainPageUrl,
  buildUploadUrl,
  withBaseUrl,
} from "../lib/fileBrowserUrls";
import type {
  FileBrowserEntry,
  FileBrowserProps,
  FieldErrors,
  MutationResult,
  UploadStatus,
} from "../lib/fileBrowserModels";

function FileBrowserBreadcrumbs({
  currentPath,
  baseUrl,
  historyMode,
  onNavigate,
}: {
  currentPath: string;
  baseUrl: string;
  historyMode: "managed" | "disabled";
  onNavigate?: (path: string) => void;
}) {
  const rootHref =
    historyMode === "managed" ? buildMainPageUrl(baseUrl, "") : buildBrowserUrl(baseUrl, "");

  return (
    <nav aria-label="breadcrumb">
      <ol className="breadcrumb mb-0">
        <li className="breadcrumb-item">
          <a
            href={rootHref}
            onClick={(event) => {
              if (!onNavigate) {
                return;
              }

              event.preventDefault();
              onNavigate("");
            }}
          >
            Root
          </a>
        </li>
        {currentPath ? (
          <li className="breadcrumb-item active" aria-current="page">
            {currentPath}
          </li>
        ) : null}
      </ol>
    </nav>
  );
}

function FolderCard({
  entry,
  baseUrl,
  historyMode,
  onNavigate,
}: {
  entry: FileBrowserEntry;
  baseUrl: string;
  historyMode: "managed" | "disabled";
  onNavigate?: (path: string) => void;
}) {
  const targetPath = entry.navigationPath ?? entry.relativePath;
  const href =
    historyMode === "managed"
      ? buildMainPageUrl(baseUrl, targetPath)
      : buildBrowserUrl(baseUrl, targetPath);

  return (
    <div className="col">
      <a
        href={href}
        className="card-link"
        onClick={(event) => {
          if (!onNavigate) {
            return;
          }

          event.preventDefault();
          onNavigate(targetPath);
        }}
      >
        <div className="card file-card text-center p-3 h-100 d-flex flex-column justify-content-center">
          <div className="file-icon folder-icon mb-2">📁</div>
          <div>{entry.name}</div>
        </div>
      </a>
    </div>
  );
}

function FileCard({
  entry,
}: {
  entry: FileBrowserEntry;
}) {
  const [copied, setCopied] = useState(false);
  const downloadUrl = entry.downloadUrl ?? "#";
  const previewUrl = entry.previewUrl ?? downloadUrl;
  const copyUrl = entry.copyUrl;

  return (
    <div className="col">
      <a href={downloadUrl} className="card-link" download>
        <div className="card file-card text-center p-3 h-100 d-flex flex-column justify-content-center">
        {entry.isImage ? (
          <img
            src={previewUrl}
            className="img-fluid rounded mb-2 shared-file-preview"
            alt="preview"
          />
        ) : (
          <div className="file-icon mb-2">📄</div>
        )}
        <div className="d-flex justify-content-center align-items-center gap-2">
          <div
            className="text-truncate"
            style={{ maxWidth: "calc(100% - 36px)" }}
            title={entry.name}
          >
            {entry.name}
          </div>
          <button
            className="btn btn-sm btn-light border"
            type="button"
            title="Copy link"
            aria-label={`Copy link for ${entry.name}`}
            onClick={async (event) => {
              event.preventDefault();
              event.stopPropagation();

              if (!copyUrl) {
                return;
              }

              const absoluteUrl = withBaseUrl(window.location.origin, copyUrl);
              await navigator.clipboard.writeText(absoluteUrl);
              setCopied(true);
              window.setTimeout(() => setCopied(false), 1500);
            }}
          >
            <i
              className={`bi ${copied ? "bi-clipboard-check text-success" : "bi-clipboard"}`}
            />
          </button>
        </div>
        </div>
      </a>
    </div>
  );
}

function UploadProgressList({ items }: { items: UploadStatus[] }) {
  if (items.length === 0) {
    return null;
  }

  return (
    <div className="mt-3" data-upload-progress>
      {items.map((item) => {
        const icon =
          item.status === "uploading"
            ? "spinner-border spinner-border-sm text-primary"
            : item.status === "success"
              ? "bi bi-check-circle-fill text-success"
              : "bi bi-x-circle-fill text-danger";

        return (
          <div key={item.id} className="d-flex align-items-center gap-2 small mb-2">
            {item.status === "uploading" ? (
              <span className={icon} role="status" aria-hidden="true" />
            ) : (
              <i className={icon} aria-hidden="true" />
            )}
            <span className="fw-semibold">{item.fileName}</span>
            <span className="text-secondary">{item.message}</span>
          </div>
        );
      })}
    </div>
  );
}

function renderFolderFieldError(fieldErrors: FieldErrors) {
  const nameErrors = fieldErrors.name;
  if (!nameErrors || nameErrors.length === 0) {
    return null;
  }

  return <div className="invalid-feedback d-block">{nameErrors[0]}</div>;
}

function renderEntry(
  entry: FileBrowserEntry,
  baseUrl: string,
  historyMode: "managed" | "disabled",
  onNavigate?: (path: string) => void,
) {
  if (entry.isDirectory) {
    return (
      <FolderCard
        key={`dir-${entry.name}`}
        entry={entry}
        baseUrl={baseUrl}
        historyMode={historyMode}
        onNavigate={onNavigate}
      />
    );
  }

  return <FileCard key={`file-${entry.name}`} entry={entry} />;
}

export function FileBrowser({
  model,
  baseUrl,
  historyMode = "managed",
  showUploadPanel = true,
  showCreateFolderPanel = true,
  onNavigate,
  onUploadFile,
  onCreateFolder,
}: FileBrowserProps) {
  const [isCreateFolderOpen, setIsCreateFolderOpen] = useState(false);
  const [folderName, setFolderName] = useState("");
  const [folderFieldErrors, setFolderFieldErrors] = useState<FieldErrors>({});
  const [folderMessage, setFolderMessage] = useState<string | null>(null);
  const [isFolderSubmitting, setIsFolderSubmitting] = useState(false);
  const [uploadStatuses, setUploadStatuses] = useState<UploadStatus[]>([]);
  const [isDragging, setIsDragging] = useState(false);
  const inputId = useMemo(
    () => `file-browser-input-${model.hubId}-${model.currentPath.replaceAll("/", "-") || "root"}`,
    [model.currentPath, model.hubId],
  );
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  async function handleCreateFolderSubmit(event: React.FormEvent<HTMLFormElement>) {
    if (!onCreateFolder) {
      return;
    }

    event.preventDefault();
    setIsFolderSubmitting(true);
    setFolderFieldErrors({});
    setFolderMessage(null);

    const result = await onCreateFolder(folderName);
    if (result.ok) {
      setFolderName("");
      setIsCreateFolderOpen(false);
      return;
    }

    setFolderFieldErrors(result.fieldErrors);
    setFolderMessage(result.message);
    setIsCreateFolderOpen(true);
    setIsFolderSubmitting(false);
  }

  async function handleFiles(fileList: FileList | File[]) {
    if (!onUploadFile) {
      return;
    }

    const files = Array.from(fileList);
    if (files.length === 0) {
      return;
    }

    const statusItems = files.map((file, index) => ({
      id: `${file.name}-${Date.now()}-${index}`,
      fileName: file.name,
      status: "uploading" as const,
      message: "Загрузка...",
    }));
    setUploadStatuses((current) => [...statusItems, ...current].slice(0, 12));

    await Promise.all(
      statusItems.map(async (statusItem, index) => {
        const result: MutationResult = await onUploadFile(files[index]);
        setUploadStatuses((current) =>
          current.map((item) =>
            item.id === statusItem.id
              ? {
                  ...item,
                  status: result.ok ? "success" : "error",
                  message: result.message,
                }
              : item,
          ),
        );
      }),
    );

    if (fileInputRef.current) {
      fileInputRef.current.value = "";
    }
  }

  return (
    <div data-react-file-browser-root data-history-mode={historyMode}>
      <div className="container my-2">
        <div className="d-flex justify-content-between align-items-center mb-3">
          <FileBrowserBreadcrumbs
            currentPath={model.currentPath}
            baseUrl={baseUrl}
            historyMode={historyMode}
            onNavigate={onNavigate}
          />
          {showCreateFolderPanel ? (
            <button
              className="btn btn-outline-primary"
              type="button"
              data-new-folder-toggle
              onClick={() => {
                setFolderMessage(null);
                setFolderFieldErrors({});
                setIsCreateFolderOpen((open) => !open);
              }}
            >
              <i className="bi bi-folder-plus me-1" /> Новая папка
            </button>
          ) : null}
        </div>

        {showCreateFolderPanel && isCreateFolderOpen ? (
          <div className="card shadow-sm mb-3" data-new-folder-panel>
            <div className="card-body">
              <div className="d-flex justify-content-between align-items-center mb-2">
                <h6 className="mb-0">Создать папку</h6>
                <button
                  type="button"
                  className="btn-close"
                  aria-label="Close"
                  data-new-folder-cancel
                  onClick={() => {
                    setIsCreateFolderOpen(false);
                    setFolderFieldErrors({});
                    setFolderMessage(null);
                  }}
                />
              </div>
              <form
                action={buildCreateFolderUrl(baseUrl, model.currentPath)}
                method="POST"
                data-new-folder-form
                onSubmit={(event) => {
                  void handleCreateFolderSubmit(event);
                }}
              >
                <div className="mb-3">
                  <label htmlFor="folderName" className="form-label">
                    Название
                  </label>
                  <input
                    type="text"
                    className={`form-control ${folderFieldErrors.name ? "is-invalid" : ""}`}
                    id="folderName"
                    name="name"
                    required
                    value={folderName}
                    onChange={(event) => {
                      setFolderName(event.target.value);
                      setFolderFieldErrors({});
                      setFolderMessage(null);
                    }}
                    placeholder="Новая папка"
                  />
                  {renderFolderFieldError(folderFieldErrors)}
                </div>
                {folderMessage && !folderFieldErrors.name ? (
                  <div className="alert alert-danger py-2" role="alert">
                    {folderMessage}
                  </div>
                ) : null}
                <div className="d-flex justify-content-end gap-2">
                  <button
                    type="button"
                    className="btn btn-outline-secondary"
                    data-new-folder-cancel
                    onClick={() => {
                      setIsCreateFolderOpen(false);
                      setFolderFieldErrors({});
                      setFolderMessage(null);
                    }}
                  >
                    Отмена
                  </button>
                  <button type="submit" className="btn btn-primary" disabled={isFolderSubmitting}>
                    {isFolderSubmitting ? "Создание..." : "Создать"}
                  </button>
                </div>
              </form>
            </div>
          </div>
        ) : null}

        {showUploadPanel ? (
          <>
            <div
              className={`dropzone ${isDragging ? "dragover" : ""}`}
              data-dropzone
              data-upload-url={buildUploadUrl(baseUrl, model.currentPath)}
              onDragOver={(event) => {
                event.preventDefault();
                setIsDragging(true);
              }}
              onDragLeave={() => {
                setIsDragging(false);
              }}
              onDrop={(event) => {
                event.preventDefault();
                setIsDragging(false);
                void handleFiles(event.dataTransfer.files);
              }}
            >
              Перетащите файлы (не более 10МБ) или{" "}
              <label htmlFor={inputId} className="text-primary cursor-pointer">
                <u>кликните для загрузки</u>
              </label>
              <input
                ref={fileInputRef}
                id={inputId}
                type="file"
                multiple
                hidden
                data-file-input
                onChange={(event) => {
                  if (event.target.files) {
                    void handleFiles(event.target.files);
                  }
                }}
              />
            </div>
            <UploadProgressList items={uploadStatuses} />
            <div className="mb-4" />
          </>
        ) : null}

        {model.errorMessage ? (
          <div className="alert alert-danger" role="alert">
            {model.errorMessage}
          </div>
        ) : null}

        <div className="row row-cols-2 row-cols-sm-3 row-cols-md-4 g-4">
          {model.entries.map((entry) => renderEntry(entry, baseUrl, historyMode, onNavigate))}
        </div>
      </div>
    </div>
  );
}
