import { buildInfo } from "../lib/buildInfo";

export function FilesShellBanner() {
  return (
    <div className="migration-banner card border-0 shadow-sm mb-3">
      <div className="card-body p-3 p-md-4">
        <div className="d-flex flex-column flex-lg-row justify-content-between gap-3 align-items-lg-center">
          <div>
            <p className="status-eyebrow mb-2">React Runtime</p>
            <h1 className="h3 mb-2">
              The files UI now runs entirely through the shared React browser
            </h1>
            <p className="text-secondary mb-0">
              Both the full page and the embedded browser use the same typed API
              layer, React-owned document shell, and shared browser component
              tree.
            </p>
          </div>
          <dl className="runtime-meta mb-0" aria-label="Current page runtime">
            <div>
              <dt>Document</dt>
              <dd>{buildInfo.runtimeOwner}</dd>
            </div>
            <div>
              <dt>Shared UI</dt>
              <dd>{buildInfo.sharedBrowserComponent}</dd>
            </div>
            <div>
              <dt>Migration</dt>
              <dd>{buildInfo.phase}</dd>
            </div>
          </dl>
        </div>
      </div>
    </div>
  );
}
