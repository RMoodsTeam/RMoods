import io
import os

from Google import Create_Service
from googleapiclient.http import MediaIoBaseDownload
from googleapiclient.errors import HttpError
import json

API_NAME = "drive"
API_VERSION = "v3"
SCOPES = ["https://www.googleapis.com/auth/drive"]


def read_model_file():
    with open("models.version", "r") as f:
        return json.load(f)


def find_folder(service, folder_name):
    query = f"name='{folder_name}' and mimeType='application/vnd.google-apps.folder'"
    response = service.files().list(q=query).execute()

    if "files" in response and len(response['files']) > 0:
        return response["files"][0]["id"]
    return None


def list_folder_contents(service, folder_id):
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
    request = service.files().get_media(fileId=file_id)

    try:
        file = io.BytesIO()
        downloader = MediaIoBaseDownload(file, request)
        done = False
        while done is False:
            status, done = downloader.next_chunk()
            print(f"Download {status.progress() * 100}")

        file.seek(0)
        if not os.path.exists(models_directory):
            os.makedirs(models_directory)

        with open(os.path.join(models_directory, file_name), "wb") as f:
            f.write(file.read())

    except HttpError as e:
        print(f"An error occurred: {e}")
        file = None


def main():
    data = read_model_file()
    service = Create_Service("client_secret_file.json", API_NAME, API_VERSION, SCOPES)

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


if __name__ == "__main__":
    main()
