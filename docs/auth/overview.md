# Identity Providers

Making sure that only specific users can access some endpoints is critical to
make sure your instance doesn't end in a flaming pile somewhere. An identity
provider (or *IdP*) is a service that both authenticates (makes sure you are you)
and authorizes (makes sure you can do something) a user.

Below are all available IdPs:

| Identity Provider | Description
| ----------------- | --- 
| [`no_auth`](auth/no-auth.md) | Allow all API calls without any form of authentication or authorization. Not recommended except in private or testing settings.
| [`preshared_key`](auth/preshared_key.md) | Use a preshared key and a timestamp to authorize and authenticate requests. Recommended for small and fast setups.

Here are some planned IdPs:

Currently, no additional IdPs are planned.
<!--
| Identity Provider | Description
| ----------------- | ---
|                   |
-->