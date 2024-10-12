import io
import os
import datetime

from httplib2 import ServerNotFoundError
from google_service import create_service
from googleapiclient.http import MediaIoBaseDownload, MediaFileUpload
from googleapiclient.errors import HttpError
from tqdm import tqdm
import json

API_NAME = "drive"
API_VERSION = "v3"
SCOPES = ["https://www.googleapis.com/auth/drive"]

if os.path.exists("client_secret_file.json"):
    SERVICE = create_service("client_secret_file.json", API_NAME,
                             API_VERSION, SCOPES)
else:
    print("client_secret_file.json file not found. Check if the file exists.")


def read_model_file():
    """
    This function reads the file with the models version.

    :return: The data read from file.
    """
    with open("models.version", "r") as f:
        return json.load(f)


def find_folder(service, folder_name):
    """
    This function finds the folder with the given name.

    :param service: The Google Drive service.
    :param folder_name: The name of the folder to find.

    :return: The folder ID if found, None otherwise.
    """
    try:
        query = (f"name='{folder_name}' and mimeType='application/"
                 f"vnd.google-apps.folder'")
        response = service.files().list(q=query).execute()
        if "files" in response and len(response['files']) > 0:
            return response["files"][0]["id"]
    except ServerNotFoundError as e:
        print(f"Server not found. Stopping looking for folder. {e}")
        return None
    except TimeoutError as e:
        print(f"Connection timed out. Stopping download. {e}")
        return None


def list_folder_contents(service, folder_id):
    """
    This function lists the contents of the folder with the given ID.

    :param service: The Google Drive service.
    :param folder_id: The ID of the folder to list.

    :return: A dictionary containing the directories or files name and ID.
    """
    try:
        query = f"'{folder_id}' in parents"
        response = service.files().list(q=query).execute()
        files = {}

        if "files" in response and len(response['files']) > 0:
            for file in response['files']:
                files[file['name']] = file['id']
        return files
    except ServerNotFoundError as e:
        print(f"Server not found. Stopping listing files. {e}")
        return None
    except TimeoutError as e:
        print(f"Connection timed out. Stopping download. {e}")
        return None


def download_file(service, file_id, file_name, models_directory):
    """
    This function downloads the file with the given ID. Based on Google Documentation
    https://developers.google.com/drive/api/guides/manage-downloads?hl=pl#python

    :param service: The Google Drive service.
    :param file_id: The ID of the file to download.
    :param file_name: The name of the file to download.
    :param models_directory: The directory to save the downloaded file.

    :return: True if the file was downloaded successfully, False otherwise.
    """
    try:
        request = service.files().get_media(fileId=file_id)
        done = False

        file = io.BytesIO()
        downloader = MediaIoBaseDownload(file, request)
        progress_bar = tqdm(total=100)

        while done is False:
            try:
                status, done = downloader.next_chunk()
                progress_bar.update(status.progress() * 100)
            except HttpError as e:
                print(f"Connection lost. Stopping download. {e}")
                return False
            except TimeoutError as e:
                print(f"Connection timed out. Stopping download. {e}")
                return False
            except ServerNotFoundError as e:
                print(f"Server not found. Stopping download. {e}")
                return False
        progress_bar.close()

        if not os.path.exists(models_directory):
            os.makedirs(models_directory)

        with open(os.path.join(models_directory, file_name), "wb") as f:
            f.write(file.read())

        if done:
            print(f"Downloaded {file_name} to {models_directory} successfully.")
            return True
    except (HttpError, ServerNotFoundError) as e:
        print(f"An error occurred: {e}")
        return False


def is_file_up_to_date(file_id, file_exists, file_path):
    """
    This function checks if the file is up to date. Comparing the created time
    in Google Drive with the one in the models.version file.

    :param file_id: The ID of the file to check.
    :param file_exists: True if the file exists, False otherwise.
    :param file_path: The path to the file.

    :return: True if the file is up to date, False otherwise.
    """
    file_created_time = (SERVICE.files().get(fileId=file_id, fields="createdTime")
                         .execute())

    if file_exists:
        online_date = datetime.datetime.strptime(
            file_created_time['createdTime'], '%Y-%m-%dT%H:%M:%S.%fZ')

        c_time = os.path.getctime(file_path)
        formatted_time = datetime.datetime.fromtimestamp(c_time).strftime(
            '%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'

        local_date = datetime.datetime.strptime(formatted_time,
                                                '%Y-%m-%dT%H:%M:%S.%fZ')
        return bool(online_date < local_date)
    else:
        return False


def get_version(folder_id, current_version, model_name):
    """
    This function is the main function of the script. It reads the file
    containing model versions, finds the folder with the given name, lists the
    contents of the folder and downloads the file.

    :param folder_id: The ID of the folder to find.
    :param current_version: The current version of the model.
    :param model_name: The name of the model.

    :return: True if the file was downloaded successfully, False otherwise.
    """
    if folder_id:
        files_with_versions = list_folder_contents(SERVICE, folder_id)
        if current_version in files_with_versions:
            folder_with_current_version_id = files_with_versions[current_version]
            list_files = list_folder_contents(SERVICE,
                                              folder_with_current_version_id)

            if len(list_files) > 0:
                file_id = list(list_files.values())[0]
                file_name = list(list_files.keys())[0]
                models_directory = f"models/{model_name}/{current_version}"
                full_models_path = f"{models_directory}/{file_name}"

                file_exists = os.path.exists(full_models_path)
                newest = is_file_up_to_date(file_id, file_exists, full_models_path)

                if not newest or not file_exists:
                    status = download_file(SERVICE, file_id, file_name,
                                           models_directory)
                    if not status:
                        print(f"An error occurred while downloading the "
                              f"file {model_name}.")
                        return False
                return True


def update_model_versions(models_names=None):
    """
    This function is the main function of the script. It reads the file
    containing model versions, finds the folder with the given name, lists the
    contents of the folder and downloads the file if it is outdated.

    :param models_names: The names of the models to update.

    :return: True if all files were updated successfully and match versions in file,
     False otherwise.
    """
    data = read_model_file()
    if models_names is None:
        models_names = []
        [models_names.append(x) for x in data]

    count_correct = 0
    errors = 0
    for model_name in models_names:
        try:
            current_version = data[model_name]
        except KeyError:
            print(f"Model {model_name} not found in the models.version file. "
                  f"Check out name of the model file.")
            errors += 1
            continue

        folder_id = find_folder(SERVICE, model_name)

        if folder_id is None:
            return False

        download_status = get_version(folder_id, current_version,
                                      model_name)
        if not download_status:
            print("Error occurred while downloading the file.")
            errors += 1
        else:
            count_correct += 1

    return bool(count_correct == len(models_names) - errors)


def get_status_information(service, parent_id='root', level=0, path='models/'
                           , print_output=True):
    """
    This function lists all folders and their subfolders recursively.

    :param service: The Google Drive service.
    :param parent_id: The ID of the parent folder.
    :param level: The current level of recursion (used for indentation).
    :param path: The path to the current folder.
    :param print_output: True if the function should print the folders and files.
    """
    try:
        query = (f"'{parent_id}' in parents and mimeType='application/"
                 f"vnd.google-apps.folder'")
        response = service.files().list(q=query).execute()
        folders = response.get('files', [])
        if not folders:
            files = list_folder_contents(service, parent_id)
            for file in files:
                file_exists = os.path.exists(path + file)
                is_not_outdated = is_file_up_to_date(files[file], file_exists,
                                                     path + file)
                if (not file_exists) or (not is_not_outdated):
                    file_conditions = "   --->   File diverged from online version"
                else:
                    file_conditions = ""

                if print_output:
                    print('  ' * level + f"File online: {file}{file_conditions}")

        for folder in folders:
            if not os.path.exists(path + folder['name']):
                folder_conditions = "   --->   Folder diverged from online version"
            else:
                folder_conditions = ""

            if print_output:
                print('  ' * level + f"Folder online: {folder['name']} "
                                                  f"{folder_conditions}")

            get_status_information(service, folder['id'], level + 1, path +
                                   folder['name'] + '/', print_output)

    except ServerNotFoundError as e:
        print(f"Server not found. Stopping listing files. {e}")
    except TimeoutError as e:
        print(f"Connection timed out. Stopping listing files. {e}")


def get_status():
    """This function prints the status of the files and folders."""
    get_status_information(SERVICE)


def create_folder(folder_name, parent_id='root'):
    """
    This function creates a folder in the Google Drive.
    :param folder_name: The name of the folder to create.
    :param parent_id: The ID of the parent folder.

    :return: The ID of the created folder.
    """
    folder_name = {
        'name': folder_name,
        'parents': [parent_id],
        'mimeType': 'application/vnd.google-apps.folder'
    }

    file = (SERVICE.files().create(
        body=folder_name)
            .execute())

    return file.get('id')


def create_file(folder_name_id, version_folder_id, file_path, file_name):
    """
    This function creates a file in the Google Drive folder.
    :param folder_name_id: The ID of the folder to create the file.
    :param version_folder_id: The ID of the version folder to create the file.
    :param file_path: The path to the file to upload.
    :param file_name: The name of the file to upload.

    :return: The file created. Or False if the folder_name_id or version_folder_id is None.
    """
    if folder_name_id is None or version_folder_id is None:
        return False

    file_metadata = {
        'name': file_name,
        'parents': [version_folder_id]
    }

    media = MediaFileUpload(file_path, resumable=True)
    file = SERVICE.files().create(body=file_metadata,
                                  media_body=media).execute()
    return file


def upload_file(folder_name, version, file_name):
    """
    This function uploads the file to the Google Drive folder.
    :param folder_name: The name of the folder to upload.
    :param version: The version of the model.
    :param file_name: The name of the file to upload.

    :return: True if the file was uploaded successfully, False otherwise.
    """
    try:
        file_path = f"models/{folder_name}/{version}/{file_name}"
        if not os.path.exists(file_path):
            print(f"File {file_name} not found. Stopping upload.")
            return False

        folder_id = find_folder(SERVICE, folder_name)
        if folder_id is None:
            folder_name_id = create_folder(folder_name)
            version_folder_id = create_folder(version, folder_name_id)
            file_create = create_file(folder_name_id, version_folder_id, file_path, file_name)

            if file_create is None:
                return False

            data = {folder_name: version}
            with open("models.version", "a") as f:
                json.dump(data, f, indent=4)

            print(f"File {file_name} uploaded successfully. Model file updated.")
            return True
        else:
            query = (f"'{folder_id}' in parents and mimeType='application/"
                     f"vnd.google-apps.folder'")
            response = SERVICE.files().list(q=query).execute()
            folders = response.get('files', [])

            version_folder_id = None
            for folder in folders:
                if folder['name'] == version:
                    version_folder_id = folder['id']
                    break

            if version_folder_id is None:
                version_folder_id = create_folder(version, folder_id)

            response_files = SERVICE.files().list(q=f"'{version_folder_id}' in parents").execute()
            files = response_files.get('files', [])

            proceed = False
            for file in files:
                if file['name'] == file_name:
                    while True:
                        answer = input(f"File {file_name} already exists Do you "
                                       f"want to overwrite the file? (y/n): ")
                        if answer == 'n':
                            return None
                        elif answer == 'y':
                            proceed = True
                            break
                        else:
                            continue
                if proceed:
                    body_value = {'trashed': True}
                    SERVICE.files().update(fileId=file['id'], body=body_value).execute()
                    break

            file_create = create_file(folder_id, version_folder_id, file_path, file_name)
            if file_create is None:
                return False

            with open("models.version", "r+") as f:
                data = json.load(f)
                data[folder_name] = version
                f.seek(0)
                json.dump(data, f, indent=4)
                f.truncate()

            print(f"File {file_name} uploaded successfully. Models file updated.")
            return True
    except HttpError as e:
        print(f"An error occurred: {e}")
        return False
    except ServerNotFoundError as e:
        print(f"Server not found. Stopping upload. {e}")
        return False
    except TimeoutError as e:
        print(f"Connection timed out. Stopping upload. {e}")
        return False


def upload_manager(folders=None):
    """
    This function manages uploads to the Google Drive folder.
    :param folders: The names of the folders to upload. If None, all folders are uploaded.
    """
    if folders is None:
        folders = read_model_file()

    data = read_model_file()
    for folder in folders:
        version = data[folder]
        files = os.listdir(f"models/{folder}/{version}")
        for file_name in files:
            print(f"Uploading {folder} model with version {version}, file {file_name}")
            update_successful = upload_file(folder, version, file_name)
            if update_successful is None:
                print(f"File {file_name} already exists. Skipping file.")
            elif not update_successful:
                print(f"Error occurred while uploading the file {file_name}.")
