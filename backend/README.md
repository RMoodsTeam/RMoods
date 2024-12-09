# RMoods Backend

## Build

For deployment, append the `release` flag.

```sh
cargo build
```

## Run

To run in debug mode, using the `.env` file:

```sh
cargo run
```

To run in release mode, using the `.env.prod` file, just add the `--release` flag.
**Be careful! Using the production DB for development is forbidden**

```sh
cargo run --release
```

## Local Environment

Your `.env` file should have all the properties as listed in the `.env.example` file.
Don't worry, the server verifies the presence of these variables at startup.

[Docker Desktop](https://www.docker.com/products/docker-desktop/) is recommended, as it's really easy to manage
containers there.

### Database setup

You'll need a local PostgreSQL database to run the backend and not cause any trouble with the production DB.

First, download the image that matches our production database version.

```sh
docker pull postgres:15.10
```

Create a docker volume to persist the data between container runs.

```sh
docker volume create rmoods-dev-pgdata
```

Run the container with the volume mounted.

```sh
docker run --name rmoods-dev-db -p 8003:5432 -e POSTGRES_USER=rmoods-dev -e POSTGRES_PASSWORD=mypasswd -e POSTGRES_DB=rmoods -v rmoods-dev-pgdata:/var/lib/postgresql/data postgres:15.10 
```

Add this to your `.env` file:

```sh
DATABASE_URL=postgres://rmoods-dev:mypasswd@localhost:8003/rmoods-db
```

The production DB URL is kept only in `.env.prod`.

To restart the container afterward:

```sh
docker start rmoods-dev-db --attach
```

With that, you should have a local database running. Now you need to configure it to match our backend code and data
schema.

### SQLx CLI

To maintain your local database, you need to use the `sqlx` CLI tool from time to time.
Refer to the [SQLx CLI documentation](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md).
Just to get you started:

```sh
sqlx database create
sqlx migrate run
```

Whenever we change the schema, you need to keep the local DB in sync. You can just drop the database and recreate it.

```sh
sqlx database drop
sqlx database create
sqlx migrate run
```

## Docker

Backend for RMoods can be run a Docker container.

```Dockerfile
docker build -t rmoods-backend .
```

After it's done building, run the container. You may want to map the default 8001 port to 9001 for differentiation.

```Dockerfile
docker run -p 9001:8001 rmoods-backend
```
