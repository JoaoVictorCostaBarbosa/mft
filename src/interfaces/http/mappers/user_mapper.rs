use crate::application::dtos::user::auth_reponse::AuthResponse;
use crate::application::dtos::user::login_request::LoginRequest;
// interfaces/http/mappers/user_mappers.rs
use crate::application::dtos::user::user_create::UserCreate;
use crate::application::dtos::user::user_response::UserResponse;
use crate::interfaces::http::dtos::user_dto::{
    AuthResponseDTO, CreateUserRequestDTO, LoginRequestDTO, UserResponseDTO,
};

pub struct UserMappers;

impl UserMappers {
    pub fn to_user_create_dto(&self, request: CreateUserRequestDTO) -> UserCreate {
        UserCreate {
            name: request.name,
            email: request.email,
            password: request.password,
        }
    }

    pub fn to_user_response_dto(&self, response: UserResponse) -> UserResponseDTO {
        UserResponseDTO {
            id: response.id,
            name: response.name.clone(),
            email: response.email.clone(),
            url_img: response.url_img,
        }
    }

    pub fn to_auth_response_dto(&self, response: AuthResponse) -> AuthResponseDTO {
        AuthResponseDTO {
            user: self.to_user_response_dto(response.user),
            access: response.access,
            refresh: response.refresh,
        }
    }

    pub fn to_login_request(&self, request: LoginRequestDTO) -> LoginRequest {
        LoginRequest {
            email: request.email,
            password: request.password,
        }
    }
}
