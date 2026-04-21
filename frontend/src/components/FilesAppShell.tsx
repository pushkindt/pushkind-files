import type { ReactNode } from "react";
import { ServiceNavbar } from "@pushkind/frontend-shell/ServiceNavbar";

import { useBootstrapSurface } from "../lib/bootstrap";
import type { FilesShellData, UserMenuItem } from "../lib/fileBrowserModels";

type FilesAppShellProps = {
  children: ReactNode;
  navigation: FilesShellData["navigation"];
  currentUserEmail: string;
  homeUrl: string;
  localMenuItems: FilesShellData["localMenuItems"];
  fetchedMenuItems: UserMenuItem[];
  banner?: ReactNode;
  flashes?: ReactNode;
};

export function FilesAppShell({
  children,
  navigation,
  currentUserEmail,
  homeUrl,
  localMenuItems,
  fetchedMenuItems,
  banner,
  flashes,
}: FilesAppShellProps) {
  const shellRef = useBootstrapSurface<HTMLDivElement>();

  return (
    <div ref={shellRef} className="files-page-shell">
      {flashes}
      <ServiceNavbar
        brand="Files"
        collapseId="filesNavbar"
        navigation={navigation}
        currentUserEmail={currentUserEmail}
        homeUrl={homeUrl}
        localMenuItems={localMenuItems}
        fetchedMenuItems={fetchedMenuItems}
        logoutAction="/logout"
      />
      {banner}
      {children}
    </div>
  );
}
