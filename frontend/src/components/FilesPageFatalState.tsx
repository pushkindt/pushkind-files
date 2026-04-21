import { ShellFatalState } from "@pushkind/frontend-shell/ShellFatalState";

type FilesPageFatalStateProps = {
  message: string;
};

export function FilesPageFatalState({ message }: FilesPageFatalStateProps) {
  return (
    <ShellFatalState
      message={message}
      serviceLabel="Files"
      title="Не удалось открыть страницу файлов"
      shellClassName="files-page-shell state-shell"
      cardClassName="card shadow-sm border-0 status-card"
      eyebrowClassName="status-eyebrow"
      titleClassName="display-6 mb-3"
    />
  );
}
