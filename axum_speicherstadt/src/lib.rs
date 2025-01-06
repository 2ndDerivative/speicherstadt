use axum::{routing::put, Router};
use crate_files::CrateFileStorage;

pub mod crate_files;
mod models;
mod publish;

pub trait AuthProvider {
    type Token;
    type User;
    type Error;
    async fn user_from_token(&self, token: Self::Token) -> Result<Self::User, Self::Error>;
}

pub struct RegistryServer<AP, CFS> {
    crate_file_storage: CFS,
    auth_provider: AP,
}
impl<AP, CFS> RegistryServer<AP, CFS> {
    pub fn new(crate_file_storage: CFS, auth_provider: AP) -> RegistryServer<AP, CFS> {
        RegistryServer {
            crate_file_storage,
            auth_provider,
        }
    }
    pub fn into_router(self) -> Router<CFS>
    where
        CFS: CrateFileStorage,
    {
        Router::new().route(
            "/api/v1/crates/new",
            put(publish::handler::<CFS>).with_state(self.crate_file_storage),
        )
    }
}
