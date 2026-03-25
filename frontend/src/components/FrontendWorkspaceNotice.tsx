type FrontendWorkspaceNoticeProps = {
  title: string;
  description: string;
};

export function FrontendWorkspaceNotice({
  title,
  description,
}: FrontendWorkspaceNoticeProps) {
  return (
    <section className="workspace-card" aria-labelledby="frontend-workspace-title">
      <p className="eyebrow">React Frontend Migration</p>
      <h1 id="frontend-workspace-title">{title}</h1>
      <p className="lede">{description}</p>
      <ul className="checklist">
        <li>Vite builds production assets into <code>assets/dist/</code>.</li>
        <li>Rust keeps serving the current Tera templates during Phase 1.</li>
        <li>Backend helpers can resolve manifest entries and built HTML files.</li>
      </ul>
    </section>
  );
}
