# RMoods NLP Backend

### Using python virtual environment
To create a python virtual environment, run the following command:

```python -m venv .venv```

To activate the virtual environment, run the following command:

bash/zsh ```source .venv/bin/activate```<br>
Windows cmd ```.venv\Scripts\activate.bat```<br>
Windows powershell ```.venv\Scripts\Activate.ps1```

To install the required packages, run the following command:

```pip install -r requirements.txt```

To exit the virtual environment, run the following command:

```deactivate```

### Running the server
To run the server, run the following command:

```python src/main.py```

### Running the tests
To run the tests, run the following command:

```python -m pytest```

## Using the models manager

#### To use the models manager, run the following command:

```bash 
./manager.sh <command> <model_names>
```

#### Manager commands:
- remote - Check the status of the models online
- install - Install the models
- clean - Delete all models from file system
- upload - Upload the models to the server
- status - Check the status of local models with Google Drive

#### Status and remote command marks:
- &#10060; - Not installed
- &#9989; - Installed
- &#10067; - Version mismatch

#### Model names is the optional parameter for upload and install:
- If no model names are provided, all models will be installed/uploaded
```bash
./manager.sh upload 
```
- To upload/install model give list of model names, separate them with a space
```bash
./manager.sh install sentiment 
```

