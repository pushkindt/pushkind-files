export function parsePathFromSearch(search: string) {
  const params = new URLSearchParams(search);
  return params.get("path") ?? "";
}

export function getPathFromLocation(location: Location) {
  return parsePathFromSearch(location.search);
}

export function updatePathInUrl(currentUrl: string, path: string) {
  const url = new URL(currentUrl);

  if (path) {
    url.searchParams.set("path", path);
  } else {
    url.searchParams.delete("path");
  }

  return url.toString();
}

export function syncBrowserHistory(path: string, replace = false) {
  const nextUrl = updatePathInUrl(window.location.href, path);
  const state = { path };

  if (replace) {
    window.history.replaceState(state, "", nextUrl);
  } else {
    window.history.pushState(state, "", nextUrl);
  }
}
