type FilesPageFatalStateProps = {
  message: string;
};

export function FilesPageFatalState({ message }: FilesPageFatalStateProps) {
  return (
    <main className="files-page-shell state-shell">
      <section className="card shadow-sm border-0 status-card">
        <div className="card-body p-4 p-md-5">
          <p className="status-eyebrow">Files</p>
          <h1 className="display-6 mb-3">Не удалось открыть страницу файлов</h1>
          <p className="text-secondary mb-0">{message}</p>
        </div>
      </section>
    </main>
  );
}
