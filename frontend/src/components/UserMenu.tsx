import type { FilesShellData } from "../lib/fileBrowserModels";

export function UserMenu({ shell }: { shell: FilesShellData }) {
  return (
    <div className="dropdown-center">
      <button
        className="btn btn-link nav-link align-items-center text-muted dropdown-toggle"
        type="button"
        data-bs-toggle="dropdown"
        aria-expanded="false"
      >
        <i className="bi bi-person-circle fs-4" />
      </button>
      <ul className="dropdown-menu dropdown-menu-end">
        <li>
          <h6 className="dropdown-header">{shell.currentUser.email}</h6>
        </li>
        <li>
          <hr className="dropdown-divider" />
        </li>
        <li>
          <a className="dropdown-item icon-link" href={shell.homeUrl}>
            <i className="bi bi-house mb-2" /> Домой
          </a>
        </li>
        <li>
          <form method="POST" action="/logout">
            <button type="submit" className="dropdown-item icon-link">
              <i className="bi bi-box-arrow-right mb-2" /> Выйти
            </button>
          </form>
        </li>
      </ul>
    </div>
  );
}
