import { UserMenuDropdown, type UserMenuItem } from "./UserMenuDropdown";
import type { FilesShellData } from "../lib/fileBrowserModels";

export function UserMenu({
  shell,
  items,
}: {
  shell: FilesShellData;
  items: UserMenuItem[];
}) {
  return (
    <UserMenuDropdown
      currentUserEmail={shell.currentUser.email}
      items={items}
      homeUrl={shell.homeUrl}
      logoutAction="/logout"
    />
  );
}
