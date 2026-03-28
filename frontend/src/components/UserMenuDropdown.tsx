export interface UserMenuItem {
  name: string;
  url: string;
}

interface UserMenuLinkItem extends UserMenuItem {
  iconClassName: string;
}

type UserMenuDropdownProps = {
  currentUserEmail: string;
  localItems: UserMenuLinkItem[];
  remoteItems: UserMenuItem[];
  logoutAction: string;
};

const LOGOUT_ITEM_NAMES = new Set([
  "logout",
  "log out",
  "sign out",
  "signout",
  "выйти",
]);

function isLogoutMenuItem(item: UserMenuItem) {
  const normalizedName = item.name.trim().toLowerCase();
  if (LOGOUT_ITEM_NAMES.has(normalizedName)) {
    return true;
  }

  try {
    const path = new URL(item.url, "https://pushkind.local").pathname.replace(
      /\/+$/,
      "",
    );
    return path === "/logout";
  } catch {
    return false;
  }
}

function buildMenuItems(
  localItems: UserMenuLinkItem[],
  remoteItems: UserMenuItem[],
): UserMenuLinkItem[] {
  return [
    ...localItems.filter((item) => !isLogoutMenuItem(item)),
    ...remoteItems
      .filter((item) => !isLogoutMenuItem(item))
      .map((item) => ({
        ...item,
        iconClassName: "bi-grid",
      })),
  ];
}

export function UserMenuDropdown({
  currentUserEmail,
  localItems,
  remoteItems,
  logoutAction,
}: UserMenuDropdownProps) {
  const menuItems = buildMenuItems(localItems, remoteItems);
  const hasNavigationItems = menuItems.length > 0;

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
          <h6 className="dropdown-header">{currentUserEmail}</h6>
        </li>
        {hasNavigationItems ? (
          <li>
            <hr className="dropdown-divider" />
          </li>
        ) : null}
        {menuItems.map((item) => (
          <li key={`${item.url}-${item.name}`}>
            <a className="dropdown-item icon-link" href={item.url}>
              <i className={`bi ${item.iconClassName} mb-2`} />
              {item.name}
            </a>
          </li>
        ))}
        <li>
          <form method="POST" action={logoutAction}>
            <button type="submit" className="dropdown-item icon-link">
              <i className="bi bi-box-arrow-right mb-2" />
              Выйти
            </button>
          </form>
        </li>
      </ul>
    </div>
  );
}
