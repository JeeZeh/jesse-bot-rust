use serde::de::DeserializeOwned;
use std::{error::Error, path::PathBuf};

use gcp_auth::{AuthenticationManager, CustomServiceAccount, Token};

pub async fn authenticate() -> Result<Token, Box<dyn Error>> {
    let credentials_path = PathBuf::from("src/credentials.json");
    let service_account = CustomServiceAccount::from_file(credentials_path)?;
    let authentication_manager = AuthenticationManager::from(service_account);
    let scopes = &[
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/firebase.database",
    ];
    Ok(authentication_manager.get_token(scopes).await?)
}

pub async fn get_json<T: DeserializeOwned>(
    path: &str,
    token: &Token,
) -> Result<T, Box<dyn Error>> {
    let url = format!(
        "https://{database_name}.firebasedatabase.app/{path}.json?access_token={access_token}",
        database_name = "jessebot-2e774-default-rtdb.europe-west1",
        path = path,
        access_token = token.as_str()
    );
    let response = reqwest::get(&url).await?;
    Ok(response.json::<T>().await?)
}
