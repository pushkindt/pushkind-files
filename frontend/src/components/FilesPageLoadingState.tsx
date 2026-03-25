export function FilesPageLoadingState() {
  return (
    <main className="files-page-shell state-shell">
      <section className="card shadow-sm border-0 status-card">
        <div className="card-body p-4 p-md-5 d-flex align-items-center gap-3">
          <div className="spinner-border text-primary" role="status" aria-hidden="true" />
          <div>
            <p className="status-eyebrow mb-2">Files</p>
            <h1 className="h3 mb-2">Подготавливаем страницу файлов</h1>
            <p className="text-secondary mb-0">
              React-загрузчик инициализирует страницу перед запуском текущего браузера файлов.
            </p>
          </div>
        </div>
      </section>
    </main>
  );
}
