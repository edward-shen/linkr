# Identity Providers

Making sure that only specific users can access some endpoints is critical to
make sure your instance doesn't end in a flaming pile somewhere. An identity
provider (or *IdP*) is a service that both authenticates (makes sure you are you)
and authorizes (makes sure you can do something) a user.

Below are all available IdPs:

| Identity Provider | Description
| ----------------- | --- 
| [`no_auth`](auth/no-auth.md) | Allow all API calls without any form of authentication or authorization. Not recommended except in private or testing settings.
| [`preshared_key`](auth/preshared_key.md) | Use a preshared key to create tokens. Tokens then are used to authenticate and authorize API calls. Easiest semi-secure method for single-users looking to have individual tokens for each application.


Here are some planned IdPs:

Currently, no additional IdPs are planned.
<!--
| Identity Provider | Description
| ----------------- | ---
|                   |
-->