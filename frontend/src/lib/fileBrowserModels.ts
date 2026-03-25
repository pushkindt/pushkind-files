export type FileBrowserEntry = {
  name: string;
  isDirectory: boolean;
  isImage: boolean;
  relativePath: string;
  navigationPath?: string;
  downloadUrl?: string;
  copyUrl?: string;
  previewUrl?: string;
};

export type FileBrowserViewModel = {
  hubId: number;
  currentPath: string;
  entries: FileBrowserEntry[];
  errorMessage?: string;
};

export type FieldErrors = Record<string, string[]>;

export type MutationSuccess = {
  ok: true;
  message: string;
};

export type MutationFailure = {
  ok: false;
  message: string;
  fieldErrors: FieldErrors;
  status: number;
};

export type MutationResult = MutationSuccess | MutationFailure;

export type UploadStatus = {
  id: string;
  fileName: string;
  status: "uploading" | "success" | "error";
  message?: string;
};

export type FilesShellData = {
  currentUser: {
    email: string;
    name: string;
    hubId: number;
  };
  homeUrl: string;
};

export type UserMenuItem = {
  name: string;
  url: string;
};

export type FileBrowserApiResponse = {
  hubId: number;
  currentPath: string;
  entries: FileBrowserEntry[];
};

export type FilesPageBootstrapData = {
  baseUrl: string;
  initialPath: string;
  runtimeOwner: "react-shell";
  sharedBrowserComponent: "FileBrowser";
  shell: FilesShellData;
  menu: UserMenuItem[];
  browser: FileBrowserApiResponse;
};

export type FileBrowserProps = {
  model: FileBrowserViewModel;
  baseUrl: string;
  historyMode?: "managed" | "disabled";
  showUploadPanel?: boolean;
  showCreateFolderPanel?: boolean;
  onNavigate?: (path: string) => void;
  onUploadFile?: (file: File) => Promise<MutationResult>;
  onCreateFolder?: (name: string) => Promise<MutationResult>;
};

export type MountFileBrowserOptions = {
  baseUrl?: string;
  historyMode?: "managed" | "disabled";
};
