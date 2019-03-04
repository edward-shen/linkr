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
|       401 | UNAUTHORIZED           | The password provided was incorrect.
|       500 | INTERNAL SERVER ERROR  | Please file a bug report.
