use crate::auth::IdentityProvider;

pub struct Provider {}

unsafe impl Send for Provider {}
unsafe impl Sync for Provider {}

impl IdentityProvider for Provider {
    /// Checks whether or not the user can log in and get a token-key pair
    fn can_get_token_key_pair(&self, _user: String, _password: String) -> bool {
        true
    }

    // Basic, core functionality.

    fn can_create_mapping(&self, _token: String) -> bool {
        print!("Hello from no_auth");
        true
    }

    fn can_delete_own_mapping(&self, _token: String) -> bool {
        true
    }

    fn can_view_own_mappings(&self, _token: String) -> bool {
        true
    }

    fn can_view_own_mapping_stats(&self, _token: String) -> bool {
        true
    }

    fn can_request_new_token(&self, _token: String) -> bool {
        true
    }

    fn can_delete_token(&self, _token: String) -> bool {
        true
    }

    // Administrative tasks

    fn can_toggle_anonymous_mode(&self, _token: String) -> bool {
        true
    }

    fn can_toggle_registration(&self, _token: String) -> bool {
        true
    }

    fn can_delete_users(&self, _token: String) -> bool {
        true
    }

    fn can_delete_tokens(&self, _token: String) -> bool {
        true
    }

    // Moderator tasks

    /// Checks whether or not the user can get all mappings
    fn can_view_all_mappings(&self, _token: String) -> bool {
        true
    }

    fn can_view_all_mapping_stats(&self, _token: String) -> bool {
        true
    }

    fn can_delete_others_mapping(&self, _token: String) -> bool {
        true
    }
}
