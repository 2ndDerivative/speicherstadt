use std::{collections::BTreeMap, path::PathBuf};

use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::error::Category;

use crate::{
    models::{dependency_kind::DependencyKind, package_name::PackageName},
    CrateFileStorage,
};

pub async fn handler<CFS: CrateFileStorage>(State(file_storage): State<CFS>, body: Bytes) -> Result<(), PublishError> {
    let (json_bytes, crate_file) = extract_body(&body)?;
    let metadata_json: PublishMetadata = match serde_json::from_slice(json_bytes) {
        Ok(md) => md,
        Err(se) => match se.classify() {
            Category::Eof | Category::Syntax => return Err(PublishError::InvalidJson),
            Category::Io => unimplemented!("can only happen for from_reader"),
            Category::Data => return Err(PublishError::InvalidMetadata(se)),
        },
    };
    todo!()
}

fn extract_body(body: &Bytes) -> Result<(&[u8], &[u8]), PublishError> {
    let Some((jlenbytes, after_first_length)) = body.split_first_chunk() else {
        return Err(PublishError::UnexpectedEOF);
    };
    let json_length = u32::from_le_bytes(*jlenbytes) as usize;
    let Some((json_bytes, after_json)) = after_first_length.split_at_checked(json_length) else {
        return Err(PublishError::UnexpectedEOF);
    };
    let Some((cflenbytes, after_length_of_crate_file)) = after_json.split_first_chunk() else {
        return Err(PublishError::UnexpectedEOF);
    };
    let length_of_crate_file = u32::from_le_bytes(*cflenbytes) as usize;
    let crate_file_data = match after_length_of_crate_file.split_at_checked(length_of_crate_file) {
        None => return Err(PublishError::UnexpectedEOF),
        Some((j, [])) => j,
        _ => return Err(PublishError::BodyTooLong),
    };
    Ok((json_bytes, crate_file_data))
}

#[derive(Debug)]
pub enum PublishError {
    UnexpectedEOF,
    BodyTooLong,
    InvalidJson,
    InvalidMetadata(serde_json::Error),
}
impl IntoResponse for PublishError {
    fn into_response(self) -> Response {
        let msg = match self {
            Self::UnexpectedEOF => "Unexpected end of input".into(),
            Self::BodyTooLong => "body longer than expected".into(),
            Self::InvalidJson => "Metadata is not a valid JSON".into(),
            Self::InvalidMetadata(jsonerr) => format!("Invalid metadata: {jsonerr}"),
        };
        (StatusCode::BAD_REQUEST, msg).into_response()
    }
}

#[derive(Deserialize)]
#[expect(dead_code)]
struct PublishMetadata {
    name: String,
    vers: String,
    deps: Vec<PublishDependencyMetadata>,
    features: BTreeMap<String, Vec<String>>,
    authors: Vec<String>,
    description: Option<String>,
    documentation: Option<String>,
    homepage: Option<String>,
    readme: Option<String>,
    readme_file: Option<PathBuf>,
    keywords: Vec<String>,
    categories: Vec<String>,
    license: Option<String>,
    license_file: Option<String>,
    repository: Option<String>,
    badges: BTreeMap<String, BTreeMap<String, String>>,
    links: Option<String>,
    rust_version: Option<String>,
}

#[derive(Deserialize)]
#[expect(dead_code)]
struct PublishDependencyMetadata {
    name: String,
    version_req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: DependencyKind,
    registry: Option<String>,
    explicit_name_in_toml: Option<String>,
}
