use std::fs;

use actix_multipart::form::tempfile::TempFile;
use pushkind_common::domain::auth::AuthenticatedUser;
use pushkind_common::routes::check_role;
use uuid::Uuid;
use validator::Validate;

use crate::SERVICE_ACCESS_ROLE;
use crate::domain::{
    EntryKind, FileName, HubId, HubStorage, RelativePath, StorageEntry, UploadRoot,
};
use crate::dto::FileEntryDto;
use crate::forms::main::CreateFolderForm;
use crate::services::{ServiceError, ServiceResult};

/// Service responsible for file system operations inside a hub's storage.
#[derive(Clone, Debug)]
pub struct FileService {
    upload_root: UploadRoot,
}

impl FileService {
    pub fn new(upload_root: UploadRoot) -> Self {
        Self { upload_root }
    }

    fn sanitize_path_param(path: Option<&str>) -> ServiceResult<RelativePath> {
        match path {
            Some(p) => RelativePath::try_from_str(p).map_err(|_| ServiceError::InvalidPath),
            None => Ok(RelativePath::root()),
        }
    }

    fn sanitize_file_name(raw: Option<String>) -> ServiceResult<FileName> {
        let generated = format!("upload-{}", Uuid::new_v4());
        let candidate = raw.unwrap_or(generated);
        FileName::try_from_str(&candidate).map_err(|_| ServiceError::InvalidFileName)
    }

    pub fn storage_for_hub(&self, hub_id: HubId) -> HubStorage {
        HubStorage::new(self.upload_root.clone(), hub_id)
    }

    fn authorize(&self, user: &AuthenticatedUser) -> ServiceResult<HubStorage> {
        if check_role(SERVICE_ACCESS_ROLE, &user.roles) {
            Ok(self.storage_for_hub(HubId::from(user.hub_id)))
        } else {
            Err(ServiceError::Unauthorized)
        }
    }

    fn ensure_hub_root(&self, storage: &HubStorage) -> ServiceResult<()> {
        fs::create_dir_all(storage.hub_root()).map_err(ServiceError::StorageSetup)
    }

    /// List entries for the given relative path, returning DTOs for rendering.
    pub fn list_entries(
        &self,
        user: &AuthenticatedUser,
        relative: Option<&str>,
    ) -> ServiceResult<Vec<FileEntryDto>> {
        let storage = self.authorize(user)?;
        let relative = Self::sanitize_path_param(relative)?;
        self.ensure_hub_root(&storage)?;

        let target_path = storage.resolve_dir(&relative);
        if !target_path.exists() {
            return Ok(vec![]);
        }
        if !target_path.is_dir() {
            return Err(ServiceError::InvalidPath);
        }

        let mut entries: Vec<StorageEntry> = fs::read_dir(&target_path)
            .map_err(ServiceError::ListEntries)?
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let file_type = entry.file_type().ok();
                let is_directory = file_type.map(|ft| ft.is_dir()).unwrap_or(false);
                let name = match FileName::try_from_str(&entry.file_name().to_string_lossy()) {
                    Ok(name) => name,
                    Err(_) => return None,
                };
                let kind = if is_directory {
                    EntryKind::Directory
                } else {
                    EntryKind::File {
                        is_image: name.is_image(),
                    }
                };

                Some(StorageEntry::new(name, kind))
            })
            .collect();

        entries.sort_by(|a, b| match (a.is_directory(), b.is_directory()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a
                .name()
                .as_str()
                .to_lowercase()
                .cmp(&b.name().as_str().to_lowercase()),
        });

        Ok(entries.into_iter().map(FileEntryDto::from).collect())
    }

    /// Create a folder (and parents) within the hub storage.
    pub fn create_folder(
        &self,
        user: &AuthenticatedUser,
        current_path: Option<&str>,
        form: &CreateFolderForm,
    ) -> ServiceResult<()> {
        form.validate()
            .map_err(|e| ServiceError::Validation(e.to_string()))?;

        let storage = self.authorize(user)?;
        self.ensure_hub_root(&storage)?;

        let current_path = Self::sanitize_path_param(current_path)?;
        let new_path = RelativePath::try_from_str(&form.name)
            .map_err(|_| ServiceError::Validation("Недопустимое имя папки".into()))?;
        let combined = current_path.join(&new_path);

        let path = storage.resolve_dir(&combined);
        fs::create_dir_all(path).map_err(ServiceError::CreateFolder)
    }

    /// Persist an uploaded file into the hub storage at the provided path.
    pub fn persist_upload(
        &self,
        user: &AuthenticatedUser,
        relative: Option<&str>,
        raw_file_name: Option<String>,
        temp_file: TempFile,
    ) -> ServiceResult<()> {
        let storage = self.authorize(user)?;
        let relative = Self::sanitize_path_param(relative)?;
        let file_name = Self::sanitize_file_name(raw_file_name)?;
        self.ensure_hub_root(&storage)?;

        let target_dir = storage.resolve_dir(&relative);
        fs::create_dir_all(&target_dir).map_err(ServiceError::SaveFile)?;

        let filepath = storage.resolve_file(&relative, &file_name);
        temp_file
            .file
            .persist(filepath)
            .map_err(|err| ServiceError::SaveFile(err.error))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    use super::*;
    use pushkind_common::domain::auth::AuthenticatedUser;
    use tempfile::{NamedTempFile, tempdir};

    fn build_service(root: PathBuf) -> FileService {
        FileService::new(UploadRoot::from(root))
    }

    #[test]
    fn list_entries_sorted_and_typed() {
        let dir = tempdir().unwrap();
        let hub_root = dir.path().join("42");
        fs::create_dir_all(&hub_root).unwrap();
        fs::create_dir(hub_root.join("b_folder")).unwrap();
        fs::write(hub_root.join("a_file.txt"), b"hello").unwrap();
        fs::write(hub_root.join("c_image.png"), b"fakepng").unwrap();

        let service = build_service(dir.path().to_path_buf());
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 42,
            name: "User".into(),
            roles: vec![SERVICE_ACCESS_ROLE.to_string()],
            exp: 0,
        };
        let entries = service.list_entries(&user, None).unwrap();

        let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "b_folder".to_string(),
                "a_file.txt".to_string(),
                "c_image.png".to_string()
            ]
        );
        assert!(entries[0].is_directory);
        assert!(!entries[1].is_directory);
        assert!(entries[2].is_image);
    }

    #[test]
    fn list_entries_missing_dir_returns_empty() {
        let dir = tempdir().unwrap();
        let service = build_service(dir.path().to_path_buf());
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 99,
            name: "User".into(),
            roles: vec![SERVICE_ACCESS_ROLE.to_string()],
            exp: 0,
        };

        let entries = service.list_entries(&user, Some("nope")).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn list_entries_rejects_parent_paths() {
        let dir = tempdir().unwrap();
        let service = build_service(dir.path().to_path_buf());
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 1,
            name: "User".into(),
            roles: vec![SERVICE_ACCESS_ROLE.to_string()],
            exp: 0,
        };

        let err = service.list_entries(&user, Some("../etc")).unwrap_err();
        assert!(matches!(err, ServiceError::InvalidPath));
    }

    #[test]
    fn create_folder_builds_nested_structure() {
        let dir = tempdir().unwrap();
        let service = build_service(dir.path().to_path_buf());
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 5,
            name: "User".into(),
            roles: vec![SERVICE_ACCESS_ROLE.to_string()],
            exp: 0,
        };
        let form = CreateFolderForm {
            name: "beta".to_string(),
        };

        service.create_folder(&user, Some("alpha"), &form).unwrap();

        let storage = service.storage_for_hub(HubId::from(5));
        assert!(
            storage
                .resolve_dir(
                    &RelativePath::try_new(PathBuf::from("alpha"))
                        .unwrap()
                        .join(&RelativePath::try_new(PathBuf::from("beta")).unwrap())
                )
                .exists()
        );
    }

    #[test]
    fn persist_upload_writes_file() {
        let dir = tempdir().unwrap();
        let service = build_service(dir.path().to_path_buf());
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 9,
            name: "User".into(),
            roles: vec![SERVICE_ACCESS_ROLE.to_string()],
            exp: 0,
        };
        let rel = RelativePath::try_new(PathBuf::from("uploads")).unwrap();
        let file_name = FileName::try_new("note.txt".to_string()).unwrap();

        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "content").unwrap();
        let temp_file = TempFile {
            file: temp,
            content_type: None,
            file_name: Some("note.txt".to_string()),
            size: 0,
        };

        service
            .persist_upload(
                &user,
                Some("uploads"),
                Some("note.txt".to_string()),
                temp_file,
            )
            .unwrap();

        let storage = service.storage_for_hub(HubId::from(9));
        let saved = storage.resolve_file(&rel, &file_name);
        assert!(saved.exists());
        let data = fs::read_to_string(saved).unwrap();
        assert!(data.contains("content"));
    }

    #[test]
    fn unauthorized_without_role() {
        let dir = tempdir().unwrap();
        let service = build_service(dir.path().to_path_buf());
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 1,
            name: "User".into(),
            roles: vec![],
            exp: 0,
        };

        let err = service.list_entries(&user, None).unwrap_err();
        assert!(matches!(err, ServiceError::Unauthorized));

        let form = CreateFolderForm {
            name: "".to_string(),
        };
        let err = service.create_folder(&user, None, &form).unwrap_err();
        assert!(matches!(err, ServiceError::Validation(_)));
    }

    #[test]
    fn create_folder_rejects_invalid_current_path() {
        let dir = tempdir().unwrap();
        let service = build_service(dir.path().to_path_buf());
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 2,
            name: "User".into(),
            roles: vec![SERVICE_ACCESS_ROLE.to_string()],
            exp: 0,
        };
        let form = CreateFolderForm {
            name: "safe".to_string(),
        };

        let err = service
            .create_folder(&user, Some("../outside"), &form)
            .unwrap_err();
        assert!(matches!(err, ServiceError::InvalidPath));
    }
}
