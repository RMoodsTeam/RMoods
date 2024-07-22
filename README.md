# RMoods Backend

## Documentation
Complete documentation for each module is available [here](https://rmoodsteam.github.io/RMoods/)

## Docker
Backend for RMoods can be run a Docker container.
```Dockerfile
docker build -t rmoods-backend .
```
After it's done building, run the container. You may want to map the default 8001 port to 9001 for differentiation.
```Dockerfile
docker run -p 9001:8001 rmoods-backend
```
