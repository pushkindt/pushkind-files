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
          iconClassName: "bi-house",
        },
      ]
    : [];

  return (
    <UserMenuDropdown
      currentUserEmail={shell.currentUser.email}
      localItems={localItems}
      remoteItems={items}
      logoutAction="/logout"
    />
  );
}
