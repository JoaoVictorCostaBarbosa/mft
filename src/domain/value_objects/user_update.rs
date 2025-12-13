#[derive(Debug, Default, Clone)]
pub struct UserUpdateFilds {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub url_img: Option<String>,
}
