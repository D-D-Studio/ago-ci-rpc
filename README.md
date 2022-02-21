# AGO CI RPC
RPC Server for continuous integration

## Table of Contents
* [Requirements](#requirements)
* [Deployment](#deployment)
  * [Local](#local)
* [Environment Variables](#environment-variables)
* [API](#api)
  * [Update CI repository](#update-ci-repository)
  * [Deploy container](#deploy-container)

## Requirements
Rust (>=1.55.0)

## Deployment
### Local
1. Build project: `cargo build` with [environment variables](#environment-variables).
2. Start server: `./target/release/ago-ci-rpc`.

## Environment Variables
| Environment Variable | Description                         | Example                            |
|----------------------|-------------------------------------|------------------------------------|
| `CI_REPO`            | URL to git repository with services | `https://git.site.com/user/ci.git` |
| `SECRET_TOKEN`       | Secret token for authorization      | `qwerty123`                        |

## API
For all methods authorization is required.

Example:
```http request
Authorization: qwerty123
```

### Update CI repository
`GET /container/update`

Update local git repository with services.

Example:
```http request
GET /contaner/update
Accept: application/json
Authorization: qwerty123
```

### Deploy container
`GET /container/{container-name}/deploy`

Deploy `{contaner-name}` container.

Example:
```http request
GET /contaner/ago-backend/deploy
Accept: application/json
Authorization: qwerty123
```

