use crate::auth::IdentityProvider;
use diesel::PgConnection;

pub struct DefaultAuth {
    connection: PgConnection,
}

impl IdentityProvider for DefaultAuth {
    /// Checks whether or not the user can log in and get a token-key pair
    fn can_get_token_key_pair(user: String, password: String) -> bool { true }

    // Basic, core functionality.

    fn can_create_mapping(token: String) -> bool { true }

    fn can_delete_own_mapping(token: String) -> bool { true }

    fn can_view_own_mappings(token: String) -> bool { true }

    fn can_view_own_mapping_stats(token: String) -> bool { true }

    fn can_request_new_token(token: String) -> bool { true }

    fn can_delete_token(token: String) -> bool { true }

    // Administrative tasks

    fn can_toggle_anonymous_mode(token: String) -> bool { true }
    
    fn can_toggle_registration(token: String) -> bool { true }
    
    fn can_delete_users(token: String) -> bool { true }

    fn can_delete_tokens(token: String) -> bool { true }

    // Moderator tasks
    
    /// Checks whether or not the user can get all mappings
    fn can_view_all_mappings(token: String) -> bool { true }

    fn can_view_all_mapping_stats(token: String) -> bool { true }

    fn can_delete_others_mapping(token: String) -> bool { true }
} 