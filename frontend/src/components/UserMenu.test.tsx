import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { UserMenu } from "./UserMenu";
import { sampleShellData } from "../lib/fileBrowserFixtures";

describe("UserMenu", () => {
  it("renders local links before fetched items and keeps logout last", () => {
    const html = renderToStaticMarkup(
      <UserMenu
        shell={sampleShellData}
        items={[
          { name: "CRM", url: "https://crm.example.com" },
          { name: "Logout", url: "https://auth.example.com/logout" },
          { name: "Store", url: "https://store.example.com" },
        ]}
      />,
    );

    const homeIndex = html.indexOf("Домой");
    const crmIndex = html.indexOf("CRM");
    const storeIndex = html.indexOf("Store");
    const logoutIndex = html.lastIndexOf("Выйти");

    expect(homeIndex).toBeGreaterThan(-1);
    expect(crmIndex).toBeGreaterThan(homeIndex);
    expect(storeIndex).toBeGreaterThan(crmIndex);
    expect(logoutIndex).toBeGreaterThan(storeIndex);
    expect(html).not.toContain("Logout");
  });
});
