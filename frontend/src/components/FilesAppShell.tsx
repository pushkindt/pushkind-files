import type { ReactNode } from "react";

import { useBootstrapSurface } from "../lib/bootstrap";

type FilesAppShellProps = {
  children: ReactNode;
  userMenu?: ReactNode;
  banner?: ReactNode;
  flashes?: ReactNode;
};

export function FilesAppShell({
  children,
  userMenu,
  banner,
  flashes,
}: FilesAppShellProps) {
  const shellRef = useBootstrapSurface<HTMLDivElement>();

  return (
    <div ref={shellRef} className="files-page-shell">
      {flashes}
      <div className="container">
        <nav className="navbar navbar-expand-sm bg-body-tertiary">
          <div className="container-fluid">
            <a className="navbar-brand" href="/">
              Files
            </a>
            <button
              className="navbar-toggler"
              type="button"
              data-bs-toggle="collapse"
              data-bs-target="#navbarSupportedContent"
              aria-controls="navbarSupportedContent"
              aria-expanded="false"
              aria-label="Toggle navigation"
            >
              <span className="navbar-toggler-icon" />
            </button>
            <div className="collapse navbar-collapse" id="navbarSupportedContent">
              <ul className="navbar-nav me-auto" />
            </div>
            {userMenu}
          </div>
        </nav>
      </div>
      {banner}
      {children}
    </div>
  );
}
