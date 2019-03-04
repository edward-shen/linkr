# Getting Started

Linkr was made with keeping it easy-to-use in mind, so most of the set-up is
done automatically. Some things, however, must still be manually done.

## Installation

### 1. Install and setup PostgreSQL

Please install PostgreSQL via your distribution's preferred method.
Then, please create a user with access to a clean database. Database migrations
are embedded into the application and will be performed automatically.

For example, one may create a `linkr` user who has access to only `linkrdb` on a
locally-hosted PostgreSQL instance. To access the database, their URL would be
```
postgres://linkr@localhost/linkrdb
```
This URL (or your setup's equivalent) is needed in the following steps.

### 2. Set up core linkr 

First, install linkr.
```bash
cargo install linkr
```

Make sure that `~/.cargo/bin` is in your path. This lets you call `linkr`
directly. Alternatively, you may always call it via an absolute route. 

An `.env` file must be in the current working directory when running `linkr`.
Create a `.env` file with the following fields:
```bash
LINKR_PASSWORD=YOUR-SECURE-PASSWORD-HERE
ROCKET_DATABASES={linkrdb={url="POSTGRES-URL-HERE"}}
```

These fields are environment variables. While not recommended, you can also simply
pass the environment variables to the command.

### 3. Choose an Identity Provider

There are a variety of Identity Providers available for linkr. Choosing one is
required.

Here are the most insecure authentication methods:

| Identity Provider | Description
| ----------------- | --- 
| [`no_auth`](auth/no-auth.md) | Allow all API calls without any form of authentication or authorization. Not recommended except in private or testing settings.
| [`preshared_key`](auth/preshared_key.md) | Use a preshared key to create tokens. Tokens then are used to authenticate and authorize API calls. Easiest semi-secure method for single-users looking to have individual tokens for each application.

For a **full list**, see the [authorization methods overview doc](auth/overview.md).

Most authorization methods have their own additional requirements to work, so
make sure to check their respective docs!

Regardless, you will need to add your authorization of your choice to the `.env`
file:
```
AUTH_METHOD=no_auth # or preshared_key, etc.
```

### 4. Run linkr

Simply call `linkr` with the `.env` in the current working directory, or with
the required environment variables set:
```bash
linkr
```

linkr does not provide any method to daemonize itself. It is out of scope and
should be handled by some other program that can handle it better, e.g. systemd.
