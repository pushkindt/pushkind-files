import type { ReactNode } from "react";
import { ServiceNoAccessPage } from "@pushkind/frontend-shell/noAccess";

import { FilesAppShell } from "../components/FilesAppShell";
import { FilesPageFatalState } from "../components/FilesPageFatalState";
import {
  fetchHubMenuItems,
  fetchNoAccessData,
  fetchShellData,
} from "../lib/filesApi";
import type { FilesShellData, UserMenuItem } from "../lib/fileBrowserModels";

function NoAccessShell({
  navigation,
  currentUserEmail,
  homeUrl,
  localMenuItems,
  fetchedMenuItems,
  children,
}: {
  navigation: FilesShellData["navigation"];
  currentUserEmail: string;
  homeUrl: string;
  localMenuItems: FilesShellData["localMenuItems"];
  fetchedMenuItems: UserMenuItem[];
  children: ReactNode;
}) {
  return (
    <FilesAppShell
      navigation={navigation}
      currentUserEmail={currentUserEmail}
      homeUrl={homeUrl}
      localMenuItems={localMenuItems}
      fetchedMenuItems={fetchedMenuItems}
    >
      {children}
    </FilesAppShell>
  );
}

export function NoAccessPage() {
  return (
    <ServiceNoAccessPage
      fetchShellData={() => fetchShellData("")}
      fetchHubMenuItems={fetchHubMenuItems}
      fetchNoAccessData={() => fetchNoAccessData("")}
      serviceLabel="Pushkind Files"
      noAccessCardClassName="files-page-shell state-shell"
      ShellComponent={NoAccessShell}
      FatalStateComponent={FilesPageFatalState}
    />
  );
}
