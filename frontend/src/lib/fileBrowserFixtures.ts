import type {
  FileBrowserApiResponse,
  FileBrowserViewModel,
  FilesShellData,
  UserMenuItem,
} from "./fileBrowserModels";

const SAMPLE_PREVIEW_DATA_URL =
  "data:image/svg+xml;utf8," +
  encodeURIComponent(
    `<svg xmlns="http://www.w3.org/2000/svg" width="640" height="420" viewBox="0 0 640 420">
      <rect width="640" height="420" fill="#f6f1e7"/>
      <rect x="42" y="42" width="556" height="336" rx="28" fill="#d96f32"/>
      <circle cx="190" cy="170" r="52" fill="#f8d36a"/>
      <path d="M120 318l122-110 86 78 68-60 124 92H120z" fill="#213547"/>
      <text x="320" y="364" text-anchor="middle" font-size="34" fill="#fff7ef" font-family="Georgia, serif">
        Pushkind
      </text>
    </svg>`,
  );

export const sampleBrowserModel: FileBrowserViewModel = {
  hubId: 42,
  currentPath: "Библиотека",
  entries: [
    {
      name: "Фото",
      isDirectory: true,
      isImage: false,
      relativePath: "Библиотека/Фото",
      navigationPath: "Библиотека/Фото",
    },
    {
      name: "poster.png",
      isDirectory: false,
      isImage: true,
      relativePath: "Библиотека/poster.png",
      downloadUrl:
        "/upload/42/%D0%91%D0%B8%D0%B1%D0%BB%D0%B8%D0%BE%D1%82%D0%B5%D0%BA%D0%B0%2Fposter.png",
      copyUrl:
        "/upload/42/%D0%91%D0%B8%D0%B1%D0%BB%D0%B8%D0%BE%D1%82%D0%B5%D0%BA%D0%B0%2Fposter.png",
      previewUrl: SAMPLE_PREVIEW_DATA_URL,
    },
    {
      name: "programme.pdf",
      isDirectory: false,
      isImage: false,
      relativePath: "Библиотека/programme.pdf",
      downloadUrl:
        "/upload/42/%D0%91%D0%B8%D0%B1%D0%BB%D0%B8%D0%BE%D1%82%D0%B5%D0%BA%D0%B0%2Fprogramme.pdf",
      copyUrl:
        "/upload/42/%D0%91%D0%B8%D0%B1%D0%BB%D0%B8%D0%BE%D1%82%D0%B5%D0%BA%D0%B0%2Fprogramme.pdf",
    },
  ],
};

export const sampleBrowserApiResponse: FileBrowserApiResponse = {
  hubId: sampleBrowserModel.hubId,
  currentPath: sampleBrowserModel.currentPath,
  entries: sampleBrowserModel.entries,
};

export const sampleShellData: FilesShellData = {
  currentUser: {
    email: "user@example.com",
    name: "Pushkind User",
    hubId: 42,
    roles: ["files"],
  },
  homeUrl: "https://auth.example.com",
  navigation: [],
  localMenuItems: [],
};

export const sampleMenuItems: UserMenuItem[] = [
  { name: "CRM", url: "https://crm.example.com" },
  { name: "Store", url: "https://store.example.com" },
];
