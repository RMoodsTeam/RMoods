import pickle
import os
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from google.auth.transport.requests import Request


def create_service(client_secret_file: str, api_name: str, api_version: str,
                   scopes: list) -> object:
    """
    Create a service object for the Google Drive API. Function is based on the Google
    API documentation: https://developers.google.com/drive/api/quickstart/python?hl=pl

    :param client_secret_file: The client secret file.
    :param api_name: The name of the API.
    :param api_version: The version of the API.
    :param scopes: The scopes needed for the API.

    :return: The service object or None.
    """
    cred = None
    pickle_file = f'token_{api_name}_{api_version}.pickle'

    if os.path.exists(pickle_file):
        with open(pickle_file, 'rb') as token:
            cred = pickle.load(token)

    if not cred or not cred.valid:
        if cred and cred.expired and cred.refresh_token:
            cred.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(client_secret_file
                                                             , scopes)
            cred = flow.run_local_server(port=8002)

        with open(pickle_file, 'wb') as token:
            pickle.dump(cred, token)

    try:
        service = build(api_name, api_version, credentials=cred)
        return service
    except Exception as e:
        print('Unable to connect.')
        print(e)
        return None
