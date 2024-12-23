import os
import fasttext
import src.globals as g
import src.utils as utils

from transformers import AutoTokenizer, AutoModelForSequenceClassification, pipeline
from keybert import KeyBERT


def load_model(model_name: str):
    """
    Load spam models function. It loads the model and tokenizer for the given
    model name.

    Args:
         model_name (str): Name of the model to load.
    """
    if model_name == "keywords":
        g.keyword_model = KeyBERT()
        return

    data = utils.read_model_file()
    model_languages = data[model_name]

    for language in model_languages:
        model_path = os.path.join("models", model_name, language)

        if not os.path.exists(model_path):
            raise RuntimeError("Model file not found")

        if model_name != "language":
            try:
                tokenizer = AutoTokenizer.from_pretrained(model_path)
                model = AutoModelForSequenceClassification.from_pretrained(
                    model_path)
            except ValueError as e:
                raise RuntimeError(f"Failed to load model: {e}")
        elif model_name == "language":
            try:
                g.language_model = fasttext.load_model(model_path + "/model.bin")
            except ValueError as e:
                raise RuntimeError(f"Failed to load model: {e}")

        if model_name == "spam":
            g.spam_pipeline = pipeline("text-classification", model=model,
                                       tokenizer=tokenizer)
        elif model_name == "sarcasm":
            g.sarcastic_pipeline = pipeline("text-classification", model=model,
                                            tokenizer=tokenizer)
        elif model_name == "sentiment":
            if language == "english":
                g.sentiment_model_english = model
                g.sentiment_tokenizer = tokenizer
            elif language == "polish":
                g.sentiment_model_polish = pipeline("text-classification",
                                                    model=model, tokenizer=tokenizer)
        elif model_name == "political":
            g.political_pipeline = pipeline("text-classification", model=model,
                                            tokenizer=tokenizer)
        elif model_name == "hate_speech":
            pipeline_name = f"hate_speech_{language}_pipeline"
            setattr(g, pipeline_name, pipeline("text-classification", model=model,
                                               tokenizer=tokenizer))
        elif model_name == "clickbait":
            g.clickbait_pipeline = pipeline("text-classification", model=model,
                                            tokenizer=tokenizer)
        elif model_name == "llm":
            g.llm_pipeline_english = pipeline("text-classification",
                                                      model=model, tokenizer=tokenizer)
