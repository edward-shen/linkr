# Identity Provider: `no_auth`

This method is not recommended in production use, as it will allow every endpoint
to be called successfully without authorization.

To be specific, setting this mode enables the following without authorization:
  - Deleting all users (not applicable if ran exclusively in `no-auth` mode)
  - Deleting all tokens (not applicable if ran exclusively in `no-auth` mode)
  - Deleting all links
  - Viewing all links
  - Viewing statistics for all links
  - Creating links

If you ever set a stricter auth mode and downgrade to `no-auth`, you are allowing
all access to all previously restricted API endpoints, so beware.

## Enabling `no_auth`

To enable this mode, simply set the following environment variable (or place this
line in the `.env` file):

```
AUTH_MODE=no_auth
```

## Using linkr in `no_auth` mode

*No extra fields are needed.*