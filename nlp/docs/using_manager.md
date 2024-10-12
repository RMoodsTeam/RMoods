# Using the models manager

To use the models manager, run the following command:

```bash 
./models.sh <command> <model_names>
```

Manager commands:
- status - Check the status of the models
- install - Install the models
- clean - Delete all models from file system
- upload - Upload the models to the server

Model names is the optional parameter for upload and install:
- If no model names are provided, all models will be installed/uploaded
```bash
./models.sh upload 
```
- To upload/install model give list of model names, separate them with a space
```bash
./models.sh install sentiment 
```