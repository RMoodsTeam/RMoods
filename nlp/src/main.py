from flask import Flask, request, jsonify
import json

app = Flask(__name__)


@app.route('/json', methods=['GET'])
def get_json_data():
    """
    This function is called when a GET request is made to the /json URL.
    It reads the JSON data from the request and write it to the file.

    :return: The data processed by certain function.
    """
    data = request.get_json()
    data = process_data(data)
    return jsonify(data)


@app.errorhandler(415)
def unsupported_media_type(error):
    """
    This function is called when a 415 error occurs.

    :param error: The error message.

    :return: The error message.
    """
    return jsonify({'error': 'Unsupported media type'}), 415


@app.errorhandler(404)
def page_not_found(error):
    """
    This function is called when a 404 error occurs.

    :param error: The error message.

    :return: The error message.
    """
    return jsonify({'error': 'Page not found'}), 404


def process_data(data):
    """
    This function processes the data and returns the result.

    :param data: The json data to be processed by function.

    :return: The processed data.
    """
    data['Modified'] = "Yes"
    return data


if __name__ == '__main__':
    app.run(host='0.0.0.0', port='8002')
