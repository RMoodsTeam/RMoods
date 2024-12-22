import string
import json
import time

def preprocess_data(input_text: str):
    """
        Preprocess input text.

        Args
            input_text (str): Input text to preprocess.

        Returns
            str: Preprocessed text.
        """
    return input_text.lower().translate(str.maketrans('', '', string.punctuation))


def confidence_output(value: float | str):
    """
       Convert confidence value to float.

       Args
           value (float | str): Confidence value.

       Returns:
            Float value of confidence.
       """
    return float(round(value, 2))


def read_model_file() -> dict:
    """
    This function reads the file with the models version.

    :return: The data read from file.
    """
    with open("version_models.json", "r") as f:
        return json.load(f)


def measure_execution_time(func) -> tuple:
    """
    Measure the execution time of a function.

    Args:
        func (function): Function to measure the execution time.

    Returns:
        tuple: Tuple with the results of the function and the execution time.
    """
    start_time = time.time()
    results = func()
    end_time = time.time()
    execution_time = round(end_time - start_time,2)

    return (results, execution_time)