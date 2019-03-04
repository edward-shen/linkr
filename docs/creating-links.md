# Creating a link

Creating a link requires you to send a `POST` request to the `/api/link` endpoint with the following fields **at the very minimum**:

| Field    | Value  | Description
| -------- | ------ | ---
| origin   | String | The point on your domain where the link should be directed from. Any value will be **URI-escaped**.
| dest     | String | The point where you want the link to go to. You **must** specify the protocol.

To make a redirected URL via cURL...
- from "your-domain.com/hello"
- to "google.com"

```bash
curl -XPOST -d "origin=hello&dest=https://google.com&password=potato" your-domain.com/api/link
```

Note that [different authentication methods](auth/overview.md) may have additional required fields.

The server will respond with one of the following:

| HTTP Code | Status                  | Meaning
| --------: | ----------------------- | ---
|       201 | CREATED                 | The link was successfully created.
|       400 | BAD REQUEST             | The hash failed to validate, or some other generic error.
|       401 | UNAUTHORIZED            | A hash must be provided for this endpoint.
|       422 | UNPROCESSABLE ENTITY    | You are missing a field, or a field failed a certain constraint.
|       425 | TOO EARLY               | The timestamp was in the future or exceeded 5 seconds.
|       409 | CONFLICT                | A link already exists on this domain.
|       500 | INTERNAL SERVER ERROR   | Something bad happened and you should file a bug report.

Depending on authentication mode, the server may never send a specific error code.

There are some limitations:
 - all characters of `origin` are either alphanumeric, "-", or "_" and cannot be "api".