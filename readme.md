# linkr

#### The self-hosted solution for url shortening or prettifying.

> I made linkr because I wanted a self-hosted solution for url shortening but all
I found were implementations in PHP. The end goal is something as robust as existing solutions but
written in safe Rust, with an emphasis on RESTful APIs first.

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

### 2. Set up linkr

Install linkr.
```bash
cargo install linkr
```

Make sure that `~/.cargo/bin` is in your path. This is required 

An `.env` file must be in the current working directory when running `linkr`.
Create a `.env` file with the following fields:
```bash
LINKR_PASSWORD=YOUR-SECURE-PASSWORD-HERE
DATABASE_URL={linkrdb={url="POSTGRES-URL-HERE"}}
```

These fields are environment variables. While not recommended, you can also simply
pass the environment variables to the command.

Finally, run linkr:
```bash
linkr
```

It is up to the end-user to daemoning linkr.

## Usage

### Creating a link

Creating a link requires you to send a `POST` request to the `/api/link` endpoint with the following fields:

| Field    | Value  | Description
| -------- | ------ | ---
| origin   | String | The point on your domain where the link should be directed from. Any value will be **URI-escaped**.
| dest     | String | The point where you want the link to go to. You **must** specify the protocol.
| password | String | The `LINKR_PASSWORD` field found in `.env` or provided as a environment variable.

To make a redirected URL via cURL...
- from "your-domain.com/hello"
- to "google.com"
- where your password is "potato"

```bash
curl -XPOST -d "origin=hello&dest=https://google.com&password=potato" your-domain.com/api/link
```

The server will respond with one of the following:

| HTTP Code | Status                  | Meaning
| --------: | ----------------------- | ---
|       201 | CREATED                 | The link was successfully created.
|       401 | UNAUTHORIZED            | The password provided was incorrect.
|       422 | UNPROCESSABLE ENTITY    | You are missing a field, or a field failed a certain constraint.
|       409 | CONFLICT                | A link already exists on this domain.
|       500 | INTERNAL SERVER ERROR   | Something bad happened and you should file a bug report.

There are some limitations:
 - all characters of `origin` are either alphanumeric, "-", or "_"

### Deleting a link

Deleting a link requires you to send a `DELETE` request to the `/api/link` endpoint with the following fields:

| Field    | Value  | Description
| -------- | ------ | ---
| origin   | String | The point on your domain to delete. Any value will be **URI-escaped**.
| password | String | The `LINKR_PASSWORD` field found in `.env` or provided as a environment variable.

To delete a URL via cURL...
- from "you-domain.com/hello"
- where your password is "potato"

```bash
curl -XDELETE -d "origin=hello&password=potato" your-domain/api/link
```

The server will respond with one of the following:

| HTTP Code | Status                 | Meaning
| --------: | ---------------------- | ---
|       200 | OK                     | The link, if it exists, was deleted.
|       401 | UNAUTHORIZED           | The password provided was incorrect.
|       500 | INTERNAL SERVER ERROR  | Please file a bug report.
