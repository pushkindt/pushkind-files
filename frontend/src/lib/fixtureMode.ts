export function isFixtureMode(search: string = window.location.search) {
  const params = new URLSearchParams(search);
  return params.get("fixture") === "1";
}
