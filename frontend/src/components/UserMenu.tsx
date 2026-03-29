import { UserMenuDropdown, type UserMenuItem } from "./UserMenuDropdown";
import type { FilesShellData } from "../lib/fileBrowserModels";

export function UserMenu({
  shell,
  items,
}: {
  shell: FilesShellData;
  items: UserMenuItem[];
}) {
  const localItems = shell.homeUrl
    ? [
        {
          name: "Домой",
          url: shell.homeUrl,
          iconClass: "bi bi-house",
        },
      ]
    : [];

  return (
    <UserMenuDropdown
      currentUserEmail={shell.currentUser.email}
      localItems={localItems}
      fetchedItems={items}
      logoutAction="/logout"
    />
  );
}
