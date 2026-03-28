use std::fs;

use reqwest::{StatusCode, header, multipart};
use serde_json::Value;

mod common;

async fn response_json(response: reqwest::Response) -> Value {
    let body = response
        .text()
        .await
        .expect("Response body should be readable.");
    serde_json::from_str(&body).expect("Response body should be valid JSON.")
}

fn form_body(fields: &[(&str, &str)]) -> String {
    fields
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&")
}

#[ignore = "local-only end-to-end test"]
#[actix_web::test]
async fn test_files_member_full_management_story() {
    let app = common::spawn_app().await;
    let client = common::build_reqwest_client();

    fs::create_dir_all(app.upload_root().join(common::HUB_ID.to_string()))
        .expect("Hub upload root should be creatable.");
    fs::create_dir_all(app.upload_root().join("777")).expect("Other hub root should be creatable.");
    fs::write(
        app.upload_root().join("777").join("other-hub.txt"),
        b"other-hub-file",
    )
    .expect("Other hub seed file should be writable.");

    common::login_as(
        &client,
        app.address(),
        "user@example.com",
        "Files User",
        common::HUB_ID,
        &["files"],
    )
    .await;

    let index_response = client
        .get(format!("{}/", app.address()))
        .send()
        .await
        .expect("Failed to request files index.");

    assert_eq!(index_response.status(), StatusCode::OK);
    let index_html = index_response
        .text()
        .await
        .expect("Files index HTML should be readable.");
    assert!(index_html.contains("files-page"));

    let iam_response = client
        .get(format!("{}/api/v1/iam", app.address()))
        .send()
        .await
        .expect("Failed to request IAM payload.");

    assert_eq!(iam_response.status(), StatusCode::OK);
    let iam_payload = response_json(iam_response).await;
    assert_eq!(iam_payload["current_user"]["email"], "user@example.com");
    assert_eq!(iam_payload["current_user"]["hub_id"], common::HUB_ID);

    let create_folder_response = client
        .post(format!("{}/folder/create", app.address()))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(form_body(&[("name", "docs")]))
        .send()
        .await
        .expect("Failed to create root folder.");

    assert_eq!(create_folder_response.status(), StatusCode::CREATED);
    assert!(app.upload_root().join("42").join("docs").is_dir());

    let nested_folder_response = client
        .post(format!("{}/folder/create?path=docs", app.address()))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(form_body(&[("name", "nested")]))
        .send()
        .await
        .expect("Failed to create nested folder.");

    assert_eq!(nested_folder_response.status(), StatusCode::CREATED);
    assert!(
        app.upload_root()
            .join("42")
            .join("docs")
            .join("nested")
            .is_dir()
    );

    let upload_image_response = client
        .post(format!("{}/files/upload", app.address()))
        .multipart(multipart::Form::new().part(
            "file",
            multipart::Part::bytes(b"png-bytes".to_vec()).file_name("poster.png"),
        ))
        .send()
        .await
        .expect("Failed to upload root image.");

    assert_eq!(upload_image_response.status(), StatusCode::OK);
    assert!(app.upload_root().join("42").join("poster.png").is_file());

    let upload_nested_response = client
        .post(format!("{}/files/upload?path=docs", app.address()))
        .multipart(multipart::Form::new().part(
            "file",
            multipart::Part::bytes(b"hello".to_vec()).file_name("report.txt"),
        ))
        .send()
        .await
        .expect("Failed to upload nested file.");

    assert_eq!(upload_nested_response.status(), StatusCode::OK);
    assert_eq!(
        fs::read(app.upload_root().join("42").join("docs").join("report.txt"))
            .expect("Nested upload should exist."),
        b"hello"
    );

    let entries_response = client
        .get(format!("{}/api/v1/files/entries", app.address()))
        .send()
        .await
        .expect("Failed to list root entries.");

    assert_eq!(entries_response.status(), StatusCode::OK);
    let entries_payload = response_json(entries_response).await;
    assert_eq!(entries_payload["hub_id"], common::HUB_ID);
    assert_eq!(entries_payload["current_path"], "");
    let entries = entries_payload["entries"]
        .as_array()
        .expect("Entries payload should be an array.");
    assert_eq!(entries[0]["name"], "docs");
    assert_eq!(entries[0]["is_directory"], true);
    assert_eq!(entries[0]["navigation_path"], "docs");
    assert_eq!(entries[1]["name"], "poster.png");
    assert_eq!(entries[1]["is_image"], true);
    assert_eq!(entries[1]["download_url"], "/upload/42/poster.png");
    assert_eq!(entries[1]["preview_url"], "/upload/42/poster.png");
    assert!(entries.iter().all(|entry| entry["name"] != "other-hub.txt"));

    let nested_entries_response = client
        .get(format!("{}/api/v1/files/entries?path=docs", app.address()))
        .send()
        .await
        .expect("Failed to list nested entries.");

    assert_eq!(nested_entries_response.status(), StatusCode::OK);
    let nested_entries_payload = response_json(nested_entries_response).await;
    let nested_entries = nested_entries_payload["entries"]
        .as_array()
        .expect("Nested entries payload should be an array.");
    assert_eq!(nested_entries_payload["current_path"], "docs");
    assert_eq!(nested_entries[0]["name"], "nested");
    assert_eq!(nested_entries[0]["is_directory"], true);
    assert_eq!(nested_entries[1]["name"], "report.txt");
    assert_eq!(
        nested_entries[1]["download_url"],
        "/upload/42/docs%2Freport.txt"
    );
    assert_eq!(nested_entries[1]["preview_url"], Value::Null);

    let download_response = client
        .get(format!("{}/upload/42/docs/report.txt", app.address()))
        .send()
        .await
        .expect("Failed to download uploaded file.");

    assert_eq!(download_response.status(), StatusCode::OK);
    assert_eq!(
        download_response
            .bytes()
            .await
            .expect("Downloaded file body should be readable."),
        b"hello"[..]
    );

    let invalid_entries_response = client
        .get(format!(
            "{}/api/v1/files/entries?path=../evil",
            app.address()
        ))
        .send()
        .await
        .expect("Failed to exercise invalid entries path.");

    assert_eq!(invalid_entries_response.status(), StatusCode::BAD_REQUEST);

    let invalid_folder_response = client
        .post(format!("{}/folder/create?path=../evil", app.address()))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(form_body(&[("name", "blocked")]))
        .send()
        .await
        .expect("Failed to exercise invalid folder path.");

    assert_eq!(invalid_folder_response.status(), StatusCode::BAD_REQUEST);

    let invalid_upload_response = client
        .post(format!("{}/files/upload?path=../evil", app.address()))
        .multipart(multipart::Form::new().part(
            "file",
            multipart::Part::bytes(b"bad".to_vec()).file_name("../escape.txt"),
        ))
        .send()
        .await
        .expect("Failed to exercise invalid upload.");

    assert_eq!(invalid_upload_response.status(), StatusCode::BAD_REQUEST);
}

#[ignore = "local-only end-to-end test"]
#[actix_web::test]
async fn test_logged_out_and_no_role_access_stories() {
    let app = common::spawn_app().await;
    let no_redirect_client = common::build_no_redirect_client();

    let logged_out_index_response = no_redirect_client
        .get(format!("{}/", app.address()))
        .send()
        .await
        .expect("Failed to request files index while logged out.");

    assert_eq!(logged_out_index_response.status(), StatusCode::SEE_OTHER);
    let logged_out_redirect = logged_out_index_response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .expect("Logged out index response should redirect.");
    assert!(logged_out_redirect.starts_with("https://users.pushkind.test/auth/signin?next="));

    let logged_out_api_response = no_redirect_client
        .get(format!("{}/api/v1/files/entries", app.address()))
        .send()
        .await
        .expect("Failed to request entries API while logged out.");

    assert_eq!(logged_out_api_response.status(), StatusCode::UNAUTHORIZED);

    let client = common::build_reqwest_client();
    common::login_as(
        &client,
        app.address(),
        "blocked@example.com",
        "Blocked User",
        common::HUB_ID,
        &[],
    )
    .await;

    let denied_index_response = client
        .get(format!("{}/", app.address()))
        .send()
        .await
        .expect("Failed to request files index without role.");

    assert_eq!(denied_index_response.status(), StatusCode::OK);
    assert_eq!(
        denied_index_response.url().as_str(),
        format!("{}/na", app.address())
    );
    let denied_index_html = denied_index_response
        .text()
        .await
        .expect("No-access HTML should be readable.");
    assert!(denied_index_html.contains("files-page"));

    let denied_iam_response = client
        .get(format!("{}/api/v1/iam", app.address()))
        .send()
        .await
        .expect("Failed to request IAM payload without role.");

    assert_eq!(denied_iam_response.status(), StatusCode::UNAUTHORIZED);

    let denied_entries_response = client
        .get(format!("{}/api/v1/files/entries", app.address()))
        .send()
        .await
        .expect("Failed to request entries payload without role.");

    assert_eq!(denied_entries_response.status(), StatusCode::UNAUTHORIZED);

    let no_access_response = client
        .get(format!("{}/api/v1/no-access", app.address()))
        .send()
        .await
        .expect("Failed to request no-access payload.");

    assert_eq!(no_access_response.status(), StatusCode::OK);
    let no_access_payload = response_json(no_access_response).await;
    assert_eq!(
        no_access_payload["current_user"]["email"],
        "blocked@example.com"
    );
}
