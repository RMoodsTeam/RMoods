import string, json , time, re
import src.globals as gl

from langcodes import tag_is_valid, Language

def preprocess_data(input_text: str):
    """
        Preprocess input text.

        Args
            input_text (str): Input text to preprocess.

        Returns
            str: Preprocessed text.
        """
    preprocessed_text = input_text.lower()
    preprocessed_text = re.sub("\n", "", preprocessed_text)
    preprocessed_text = re.sub("\t", "", preprocessed_text)

    preprocessed_text = preprocessed_text.replace(u'\xa0', u' ')

    return preprocessed_text.translate(str.maketrans('', '', string.punctuation))


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

    return results, execution_time

def detect_language(text):
    prediction = gl.language_model.predict(text, k=1)
    lang_tag = prediction[0][0].rsplit("_")[-2]
    if tag_is_valid(lang_tag):
        lang_name = Language.get(lang_tag).display_name("en")
        return lang_name
    return lang_tag
