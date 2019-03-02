use crate::auth::IdentityProvider;
use diesel::PgConnection;

pub struct DefaultAuth {
    _connection: PgConnection,
}

impl IdentityProvider for DefaultAuth {
    /// Checks whether or not the user can log in and get a token-key pair
    fn can_get_token_key_pair(_user: String, _password: String) -> bool {
        true
    }

    // Basic, core functionality.

    fn can_create_mapping(_token: String) -> bool {
        true
    }

    fn can_delete_own_mapping(_token: String) -> bool {
        true
    }

    fn can_view_own_mappings(_token: String) -> bool {
        true
    }

    fn can_view_own_mapping_stats(_token: String) -> bool {
        true
    }

    fn can_request_new_token(_token: String) -> bool {
        true
    }

    fn can_delete_token(_token: String) -> bool {
        true
    }

    // Administrative tasks

    fn can_toggle_anonymous_mode(_token: String) -> bool {
        true
    }

    fn can_toggle_registration(_token: String) -> bool {
        true
    }

    fn can_delete_users(_token: String) -> bool {
        true
    }

    fn can_delete_tokens(_token: String) -> bool {
        true
    }

    // Moderator tasks

    /// Checks whether or not the user can get all mappings
    fn can_view_all_mappings(_token: String) -> bool {
        true
    }

    fn can_view_all_mapping_stats(_token: String) -> bool {
        true
    }

    fn can_delete_others_mapping(_token: String) -> bool {
        true
    }
}
