use axum::{extract::State, Json};
use domain::UsersManager;
use macros::router;
use serde::{Deserialize, Serialize};
use util::jwt::encode_token;
use validator::Validate;

use crate::{app_error::LoginError, app_request::ValidatedJson, app_response::AppError, AppState};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct LoginForm {
    #[validate(length(min = 2, max = 20, code = "login-valid-username"))]
    username: String,
    #[validate(length(min = 8, max = 32, code = "login-valid-password"))]
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[tracing::instrument(skip(state))]
#[router(path = "/api/users/login", method = "post")]
async fn login(
    State(state): State<AppState>,
    ValidatedJson(login_form): ValidatedJson<LoginForm>,
) -> Result<Json<LoginResponse>, AppError> {
    let manager = &state.users_manager;

    let tuple = manager
        .verify_user(login_form.username.clone(), login_form.password)
        .await?;

    let (pass, user) = tuple;

    match pass {
        true => {
            let token = encode_token(user.id, login_form.username, user.roles);
            Ok(LoginResponse { token }.into())
        }
        false => Err(AppError::from(LoginError::WrongPassword)),
    }
}
