pub mod no_auth;
pub mod preshared_key;

pub struct IdP<'a> {
    pub provider: &'a IdentityProvider,
}

unsafe impl Send for IdP<'_> {}
unsafe impl Sync for IdP<'_> {}

/// An identity provider must provide authentication for every end point
pub trait IdentityProvider {
    /// Checks whether or not the user can log in and get a token-key pair
    fn can_get_token_key_pair(&self, user: String, password: String) -> bool;

    // Basic, core functionality.

    fn can_create_mapping(&self, token: String) -> bool;

    fn can_delete_own_mapping(&self, token: String) -> bool;

    fn can_view_own_mappings(&self, token: String) -> bool;

    fn can_view_own_mapping_stats(&self, token: String) -> bool;

    fn can_request_new_token(&self, token: String) -> bool;

    fn can_delete_token(&self, token: String) -> bool;

    // Administrative tasks

    fn can_toggle_anonymous_mode(&self, token: String) -> bool;

    fn can_toggle_registration(&self, token: String) -> bool;

    fn can_delete_users(&self, token: String) -> bool;

    fn can_delete_tokens(&self, token: String) -> bool;

    // Moderator tasks

    /// Checks whether or not the user can get all mappings
    fn can_view_all_mappings(&self, token: String) -> bool;

    fn can_view_all_mapping_stats(&self, token: String) -> bool;

    fn can_delete_others_mapping(&self, token: String) -> bool;

    // Meta required fields
    fn get_key(&self) -> Option<String>;
}
