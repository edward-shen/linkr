# Deleting a link

Deleting a link requires you to send a `DELETE` request to the `/api/link` endpoint with the following fields **at the very minimum**:

| Field    | Value  | Description
| -------- | ------ | ---
| origin   | String | The point on your domain to delete. Any value will be **URI-escaped**.

To delete a URL via cURL...
- from "your-domain.com/hello"

```bash
curl -XDELETE -d "origin=hello&password=potato" your-domain/api/link
```

Note that [different authentication methods](auth/overview.md) may have additional required fields.

The server will respond with one of the following:

| HTTP Code | Status                 | Meaning
| --------: | ---------------------- | ---
|       200 | OK                     | The link, if it exists, was deleted.
|       400 | BAD REQUEST            | The hash failed to validate, or some other generic error.
|       401 | UNAUTHORIZED           | A hash must be provided for this endpoint.
|       425 | TOO EARLY              | The timestamp was in the future or exceeded 5 seconds.
|       500 | INTERNAL SERVER ERROR  | Please file a bug report.

Depending on authentication mode, the server may never send a specific error code.