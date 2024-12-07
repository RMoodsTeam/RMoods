import string

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
