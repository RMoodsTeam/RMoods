# Installation guide 

## Using python virtual environment
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

## Running the server
To run the server, run the following command:

```python src/main.py```

## Running the tests
To run the tests, run the following command:

```python -m pytest```