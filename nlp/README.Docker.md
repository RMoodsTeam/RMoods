## Building and running your application

When you're ready, build your application by running:
```docker build -t rmoods-nlp .```

The ```-t``` parameter refers to tag, which is the name of the image. 
You can replace ``rmoods-nlp`` with any name you like.

After the build completes, you can run your application using the following command:
```docker run --name rmoods-nlp -d -v $(pwd)/models:/models rmoods-nlp```

The ```-d``` parameter runs the container in detached mode, which means the container 
runs in the background ```--name``` refers to docker name, it can be used 
instead of container id. To prevent copying the model to the container, we use the 
```-v``` parameter to mount the models directory to the container.

If you want to open docker image console run:
```docker exec -it rmoods-nlp /bin/bash```

```rmoods-nlp``` is the name of the container. If we did not specify a name,
Docker would have assigned a random name to the container. To check the name of the container, run:
```docker ps```

To stop the container, run:
```docker stop rmoods-nlp```

We can also run use docker compose to build and run the application.
```docker-compose up```

## References
* [Docker's Python guide](https://docs.docker.com/language/python/)