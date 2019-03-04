use crate::auth::IdentityProvider;

pub struct Provider {
    pub key: String,
}

unsafe impl Send for Provider {}
unsafe impl Sync for Provider {}

impl IdentityProvider for Provider {
    // Checks whether or not the user can log in and get a token-key pair

    /// Token is pre-shared, so login isn't necessary.
    fn can_get_token_key_pair(&self, _user: String, _password: String) -> bool {
        false
    }

    // Basic, core functionality.

    fn can_create_mapping(&self, token: String) -> bool {
        print!("henmlpo");
        token == self.key
    }

    fn can_delete_own_mapping(&self, token: String) -> bool {
        token == self.key
    }

    fn can_view_own_mappings(&self, token: String) -> bool {
        token == self.key
    }

    fn can_view_own_mapping_stats(&self, token: String) -> bool {
        token == self.key
    }

    // User should be able to request new tokens (e.g. for program-granularity)
    fn can_request_new_token(&self, token: String) -> bool {
        token == self.key
    }

    // User cannot delete the master pre-shared key, but can delete other tokens.
    fn can_delete_token(&self, token: String) -> bool {
        token == self.key
    }

    // Administrative tasks
    // In single user mode, this makes no sense to have enabled.

    fn can_toggle_anonymous_mode(&self, _token: String) -> bool {
        false
    }

    fn can_toggle_registration(&self, _token: String) -> bool {
        false
    }

    fn can_delete_users(&self, _token: String) -> bool {
        false
    }

    fn can_delete_tokens(&self, _token: String) -> bool {
        false
    }

    // Moderator tasks

    fn can_view_all_mappings(&self, _token: String) -> bool {
        false
    }

    fn can_view_all_mapping_stats(&self, _token: String) -> bool {
        false
    }

    fn can_delete_others_mapping(&self, _token: String) -> bool {
        false
    }
}
