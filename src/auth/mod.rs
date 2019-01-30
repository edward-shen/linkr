
pub struct TokenKeyPair {
    pub token: String,
    pub key: String,
}

pub trait IdentityProvider {
    fn authenticate_user(user: String, password: String) -> Option<TokenKeyPair>;
}
