from flask import Flask, request, jsonify
import json

app = Flask(__name__)

@app.route('/', methods=['GET', 'POST'])
def home():
    """
    This function is called when a GET or POST request is made to the / URL.

    Returns:
        str: A message indicating the success of the operation.
    """
    return 'Hello, World!'


@app.route('/json', methods=['GET'])
def get_json_file():
    """
    This function is called when a GET request is made to the /json URL.
    It reads the JSON data from the request and write it to the file.

    Returns:
        dict: The data processed by certain function.
    """
    data = request.get_json()
    with open('data.json', 'w') as f:
        f.write(json.dumps(data))
    
    data = process_data(data)
    return jsonify(data)


def process_data(data):
    """
    This function processes the data and returns the result.

    Args:
        data (dict): The data to be processed.

    Returns:
        dict: The processed data.
    """
    data['Modified'] = "Yes"
    return data


if __name__ == '__main__':
    app.run(host='0.0.0.0', port='8010')
