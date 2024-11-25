from statsmodels.graphics.tukeyplot import results
from transformers import AutoTokenizer, AutoModelForSequenceClassification
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
sentiment_tokenizer = None
sarcastic_model = None
sarcastic_tokenizer = None
spam_model = None
spam_tokenizer = None
political_model = None
political_tokenizer = None
hate_speech_model = None
hate_speech_tokenizer = None


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

def load_model(model_name: str):
    """
    Load spam models function. It loads the model and tokenizer for the given
    model name.

    :param model_name: Name of the model to load.
    """
    global spam_model, spam_tokenizer
    global sarcastic_model, sarcastic_tokenizer
    global sentiment_model, sentiment_tokenizer
    global political_model, political_tokenizer
    global hate_speech_model, hate_speech_tokenizer

    model_path = os.path.join("models", model_name, "v1")

    if not os.path.exists(model_path):
        raise RuntimeError("Model file not found")

    try:
        tokenizer = AutoTokenizer.from_pretrained(model_path)
        model = AutoModelForSequenceClassification.from_pretrained(
            model_path)
    except ValueError as e:
        raise RuntimeError(f"Failed to load model: {e}")

    if model_name == "spam":
        spam_model = model
        spam_tokenizer = tokenizer
    elif model_name == "sarcasm":
        sarcastic_model = model
        sarcastic_tokenizer = tokenizer
    elif model_name == "sentiment":
        sentiment_model = model
        sentiment_tokenizer = tokenizer
    elif model_name == "political":
        political_model = model
        political_tokenizer = tokenizer
    elif model_name == "hate_speech":
        hate_speech_model = model
        hate_speech_tokenizer = tokenizer


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
    load_model("sentiment")

    print("Loading sarcastic model.")
    load_model("sarcasm")

    print("Loading spam model.")
    load_model("spam")

    print("Loading politics model.")
    load_model("political")

    print("Loadaing hate_speech model.")
    load_model("hate_speech")

    yield
    print("Application is shutting down.")


app = FastAPI(lifespan=lifespan)


class TextRequest(BaseModel):
    """Request model for text"""
    text: List[str]


@app.post("/sentiment", response_model=dict)
async def get_sentiment(request: TextRequest):
    """
    Get sentiment of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    if sentiment_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        0: "Sadness",
        1: "Joy",
        2: "Love",
        3: "Anger",
        4: "Fear",
        5: "Surprise"
    }

    results = []
    for text in request.text:
        tokenized_text = sentiment_tokenizer([preprocess_data(text)],
                                             padding=True, truncation=True,
                                             max_length=128, return_tensors="pt")
        output = sentiment_model(**tokenized_text)
        probs = output.logits.softmax(dim=-1).tolist()[0]
        confidence = max(probs)
        prediction = probs.index(confidence)
        text_values = {
            "prediction": labels[prediction],
            "confidence": f'{confidence:.2f}'
        }
        results.append(text_values)

    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/language", response_model=dict)
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


@app.post("/sarcasm", response_model=dict)
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
            "confidence": f'{confidence:.2f}'
        }
        results.append(text_values)

    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/keywords")
async def get_keywords(request: TextRequest):
    """
    Get keywords of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"keywords": text}


@app.post("/spam")
async def get_spam(request: TextRequest):
    """
    Get spam of the text. 0 is not spam, 1 is spam.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    if spam_model is None or spam_tokenizer is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        tokenized_text = spam_tokenizer([text],
                                        padding=True, truncation=True,
                                        max_length=128, return_tensors="pt")
        output = spam_model(**tokenized_text)
        probs = output.logits.softmax(dim=-1).tolist()[0]
        confidence = max(probs)
        prediction = probs.index(confidence)
        text_values = {
            "prediction": prediction,
            "confidence": f'{confidence:.2f}'
        }
        results.append(text_values)

    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/politics")
async def get_politics(request: TextRequest):
    """
    Get politics of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    # Model accuracy may not hold up on pieces of text longer than a tweet.
    if political_model is None or political_tokenizer is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    label = {
        0: "Republican",
        1: "Democrat",
    }

    results = []
    for text in request.text:
        tokenized_text = political_tokenizer([preprocess_data(text)],
                                             padding=True, truncation=True,
                                             max_length=128, return_tensors="pt")
        output = political_model(**tokenized_text)
        probs = output.logits.softmax(dim=-1).tolist()[0]
        confidence = max(probs)
        prediction = probs.index(confidence)
        text_values = {
            "prediction": label[prediction],
            "confidence": f'{confidence:.2f}'
        }
        results.append(text_values)

    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/hate-speach")
async def get_hate_speech(request: TextRequest):
    """
    Get hate speech of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    # Do zamieszczenia bibliografie z linku
    # https: // huggingface.co / Hate - speech - CNERG / dehatebert - mono - english
    #Pamiętamy
    if hate_speech_model is None or hate_speech_tokenizer is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    label = {
        0: "Not-hate",
        1: "Hate"
    }
    results = []
    for text in request.text:
        tokenized_text = hate_speech_tokenizer([preprocess_data(text)],
                                               padding=True, truncation=True,
                                               max_length=128, return_tensors="pt")
        output = hate_speech_model(**tokenized_text)
        probs = output.logits.softmax(dim=-1).tolist()[0]
        confidence = max(probs)
        prediction = probs.index(confidence)
        text_values = {
            "prediction": label[prediction],
            "confidence": f'{confidence:.2f}'
        }
        results.append(text_values)

    return {"metadata": {"generated_in": 0.0}, "results": results}


@app.post("/clickbait")
async def get_clickbait(request: TextRequest):
    """
    Get clickbait of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"clickbait": text}


@app.post("/troll")
async def get_troll(request: TextRequest):
    """
    Get troll of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"troll": text}
