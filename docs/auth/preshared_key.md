# Identity Provider: `preshared_key`

Using a preshared key is an option for users who wish to have a relatively
simple yet secure way of using linkr. Communicating does get a little tricky,
as you are now required to produce a HMAC SHA256 hash alongside your request,
but this is available effectively in every language or on platform.

## Enabling `preshared_key`

To enable this mode, simply set the following environment variable (or place this
line in the `.env` file):

```
AUTH_MODE=preshared_key
PRESHARED_TOKEN=<YOUR KEY>
```

While this can be anything, it is highly recommended to use a long, strong
password or passphrase are your preshared_key.

## Using linkr in `preshared_key` mode

The `/tools/` contains `psk_create_link` and `psk_delete_link`, which should be
used to test out this mode.

**The parameters must be in a certain order**. Failure to send parameters in
the following order will yield a failed result:

```
origin=v1[&dest=v2]&ts=v3&hash=v4
```

`ts` is the time in seconds since unix epoch, and `hash` is the HMAC SHA256 hash
of the following string:

```
origin=v1[&dest=v2]&ts=v3
```

Note that `dest` is only required if you are creating links, and must be left out
for deleting links.

Here are some concrete examples:
```bash
# Creating a link
curl -XPOST -d "origin=asdff&dest=hosd&ts=1551681791&hash=a84ee951112f89feaa34fe32d052c17187edbc2fb7ec35dfe710d06b5b17ad05" localhost:8000/api/link

# Deleting a link
# Notice how the dest parameter is omitted.
curl -XDELETE -d "origin=asdf&ts=1551681861&hash=155b46c17892125e50a5284e916b8fee2f039b7481dd1ba16b117d80c6ffbd26" localhost:8000/api/link

# This won't work because the parameters are out of order!
curl -XDELETE -d "ts=1551681861&origin=asdf&hash=155b46c17892125e50a5284e916b8fee2f039b7481dd1ba16b117d80c6ffbd26" localhost:8000/api/link
```