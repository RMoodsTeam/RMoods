# RMoods Backend

## Build
For deployment, append the `release` flag.
```sh
cargo build
```


## Run
```sh
cargo run
```
### Environment
Your `.env` file should look like this:
```
CLIENT_ID=***
CLIENT_SECRET=***
DATABASE_URL=***
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
