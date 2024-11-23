from transformers import pipeline, AutoTokenizer, AutoModelForSequenceClassification
from langcodes import tag_is_valid, Language
from contextlib import asynccontextmanager
from typing import List
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from src.version_checker import update_model_versions

import fasttext
import os
import string


language_model = None
sentiment_model = None
sarcastic_model = None
sarcastic_tokenizer = None


def load_language_model():
    """Load language model."""
    global language_model
    language_model_path = os.path.join("models", "language", "v1", "model.bin")
    if not os.path.exists(language_model_path):
        raise RuntimeError("Model file not found")

    try:
        language_model = fasttext.load_model(language_model_path)
    except ValueError as e:
        raise RuntimeError(f"Failed to load model: {e}")


def load_sentiment_model():
    """Load sentiment model."""
    global sentiment_model
    sentiment_model_path = os.path.join("models", "sentiment", "v1")
    if not os.path.exists(sentiment_model_path):
        raise RuntimeError("Model file not found")

    try:
        sentiment_tokenizer = AutoTokenizer.from_pretrained(sentiment_model_path)
        sentiment_model = pipeline("sentiment-analysis",
                                   model=sentiment_model_path,
                                   tokenizer=sentiment_tokenizer)
    except ValueError as e:
        raise RuntimeError(f"Failed to load model: {e}")


def load_sarcastic_model():
    """Load sarcastic model."""
    global sarcastic_model
    global sarcastic_tokenizer
    sarcastic_model_path = os.path.join("models", "sarcasm", "v1")
    if not os.path.exists(sarcastic_model_path):
        raise RuntimeError("Model file not found")

    try:
        sarcastic_tokenizer = AutoTokenizer.from_pretrained(sarcastic_model_path)
        sarcastic_model = AutoModelForSequenceClassification.from_pretrained(
            sarcastic_model_path)
    except ValueError as e:
        raise RuntimeError(f"Failed to load model: {e}")


def preprocess_data(input_text):
    """
    Preprocess input text.

    :param input_text: Input text to preprocess.
    :return: Preprocessed text.
    """
    return input_text.lower().translate(str.maketrans('', '', string.punctuation))


@asynccontextmanager
async def lifespan(application: FastAPI):
    """
    Context manager for the lifespan of the application.

    :param application: FastAPI application.
    """
    print("Application is starting.")
    update_model_versions()

    print("Loading language model.")
    load_language_model()

    print("Loading sentiment model.")
    load_sentiment_model()

    print("Loading sarcastic model.")
    load_sarcastic_model()

    yield
    print("Application is shutting down.")


app = FastAPI(lifespan=lifespan)


class TextRequest(BaseModel):
    """Request model for text"""
    text: List[str]


@app.post("/report/sentiment", response_model=dict)
async def get_sentiment(request: TextRequest):
    """
    Get sentiment of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    if sentiment_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        prediction = sentiment_model.predict(text)

        text_values = {
            "label": prediction[0]["label"],
            "score": f'{prediction[0]["score"]:.2f}'
        }
        results.append(text_values)

    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/report/language", response_model=dict)
async def get_language(request: TextRequest):
    """
    Get language of the texts from request.

    :param request: Request from server in json format

    :return: Dictionary with language and predictions. Each of them is a list, with
            list as many text as in the request array.
    """
    if language_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        languages = []
        prediction = language_model.predict(text, k=2)
        
        for predicted_lang in prediction[0]:
            lang_tag = predicted_lang.rsplit("_")[-2]
            if tag_is_valid(lang_tag):
                lang_name = Language.get(lang_tag).display_name("en")
                languages.append(lang_name)
            else:
                languages.append(lang_tag)

        predictions = [f"{y:.2f}" for y in prediction[1]]

        text_values = {
            "language": languages,
            "predicted": predictions
        }
        results.append(text_values)
    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/report/sarcasm", response_model=dict)
async def get_sarcasm(request: TextRequest):
    """
    Get sarcasm of the text. 0 is not sarcastic, 1 is sarcastic.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    if sarcastic_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        tokenized_text = sarcastic_tokenizer([preprocess_data(text)],
                                             padding=True, truncation=True,
                                             max_length=128, return_tensors="pt")
        output = sarcastic_model(**tokenized_text)
        probs = output.logits.softmax(dim=-1).tolist()[0]
        confidence = max(probs)
        prediction = probs.index(confidence)
        text_values = {
            "prediction": prediction,
            "confidence": confidence
        }
        results.append(text_values)

    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/report/keywords")
async def get_keywords(request: TextRequest):
    """
    Get keywords of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"keywords": text}


@app.post("/report/spam")
async def get_spam(request: TextRequest):
    """
    Get spam of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"spam": text}


@app.post("/report/politics")
async def get_politics(request: TextRequest):
    """
    Get politics of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"politics": text}


@app.post("/report/hate-speach")
async def get_hate_speech(request: TextRequest):
    """
    Get hate speech of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"hate-speech": text}


@app.post("/report/clickbait")
async def get_clickbait(request: TextRequest):
    """
    Get clickbait of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"clickbait": text}


@app.post("/report/troll")
async def get_troll(request: TextRequest):
    """
    Get troll of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"troll": text}
