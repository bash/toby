# HTTP API

The HTTP API is exposed on port `8629` by default.

## Authorization

Authorization is done throught the `Authorization` header.
It is prefixed with `Token ` and contains the token and secret separated with a colon.

```
Authorization: Token travis:0dbCv0tPHHNZ3KMLiWuPO
```

## Endpoints

### `POST /v1/jobs/:project`

This endpoint triggers a job for the given project. It will return `403 Forbidden` if either the project isn't configured or when the access checks fail.

```sh
curl -X POST http://toby.server:8629/v1/jobs/dreams \
     -H "Authorization: Token travis:$TOBY_SECRET"
```