import io
import os

from google_service import create_service
from googleapiclient.http import MediaIoBaseDownload
from googleapiclient.errors import HttpError
from tqdm import tqdm
import json

API_NAME = "drive"
API_VERSION = "v3"
SCOPES = ["https://www.googleapis.com/auth/drive"]


def read_model_file():
    """
    This function reads the models.version file and returns the data.

    Returns:
        :return: The data from the models.version file.
    """
    with open("models.version", "r") as f:
        return json.load(f)


def find_folder(service, folder_name):
    """
    This function finds the folder with the given name.

    Inputs:
        :param service: The Google Drive service.
        :param folder_name: The name of the folder to find.

    Returns:
        :return: The folder ID if found, None otherwise.
    """
    query = f"name='{folder_name}' and mimeType='application/vnd.google-apps.folder'"
    response = service.files().list(q=query).execute()

    if "files" in response and len(response['files']) > 0:
        return response["files"][0]["id"]
    return None


def list_folder_contents(service, folder_id):
    """
    This function lists the contents of the folder with the given ID.

    Inputs:
        :param service: The Google Drive service.
        :param folder_id: The ID of the folder to list.

    Returns:
        :return: A dictionary containing the directories or files name and ID.
    """
    query = f"'{folder_id}' in parents"
    response = service.files().list(q=query).execute()
    files = {}

    if "files" in response and len(response['files']) > 0:
        for file in response['files']:
            files[file['name']] = file['id']
        return files
    else:
        return files


def download_file(service, file_id, file_name, models_directory):
    """
    This function downloads the file with the given ID.
    Based on Google Documentation https://developers.google.com/drive/api/guides/manage-downloads?hl=pl#python

    Inputs:
        :param service: The Google Drive service.
        :param file_id: The ID of the file to download.
        :param file_name: The name of the file to download.
        :param models_directory: The directory to save the downloaded file.
    """
    request = service.files().get_media(fileId=file_id)

    try:
        file = io.BytesIO()
        downloader = MediaIoBaseDownload(file, request)
        done = False
        pbar = tqdm(total=100)

        while done is False:
            status, done = downloader.next_chunk()
            pbar.update(status.progress() * 100)
        pbar.close()
        print(f"Downloaded {file_name} to {models_directory} successfully.")

        file.seek(0)
        if not os.path.exists(models_directory):
            os.makedirs(models_directory)

        with open(os.path.join(models_directory, file_name), "wb") as f:
            f.write(file.read())

    except HttpError as e:
        print(f"An error occurred: {e}")
        file = None
    return file.getvalue()


def get_version():
    """
    This function is the main function of the script. It reads the models.version file, finds the folder with the given
    name, lists the contents of the folder and downloads the file.
    """
    data = read_model_file()
    service = create_service("client_secret_file.json", API_NAME, API_VERSION, SCOPES)

    for key in data:
        folder_id = find_folder(service, key)
        current_version = data[key]

        if folder_id:
            files_with_versions = list_folder_contents(service, folder_id)
            if current_version in files_with_versions:
                folder_with_current_version_id = files_with_versions[current_version]
                list_files = list_folder_contents(service, folder_with_current_version_id)

                if len(list_files) > 0:
                    file_id = list(list_files.values())[0]
                    file_name = list(list_files.keys())[0]
                    models_directory = f"models/{key}/{current_version}"
                    download_file(service, file_id, file_name, models_directory)
