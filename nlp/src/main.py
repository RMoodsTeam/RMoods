from fastapi import FastAPI, HTTPException
from src.version_checker import update_model_versions
from pydantic import BaseModel
from typing import List
from contextlib import asynccontextmanager
import os
import fasttext

language_model = None

def load_language_model():
    """
    Load language model.
    """
    global language_model
    language_model_path = os.path.join("models", "language", "v1", "model.bin")
    if not os.path.exists(language_model_path):
        raise RuntimeError("Model file not found")

    try:
        language_model = fasttext.load_model(language_model_path)
    except ValueError as e:
        raise RuntimeError(f"Failed to load model: {e}")


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

    yield
    print("Application is shutting down.")


app = FastAPI(lifespan=lifespan)


class TextRequest(BaseModel):
    """Request model for text"""
    text: List[str]


@app.post("/report/sentiment")
async def get_sentiment(request: TextRequest):
    """
    Get sentiment of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"sentiment": text}


@app.post("/report/language", response_model=dict)
async def get_language(request: TextRequest):
    """
    Get language of the texts from request.

    :param request: Request from server in json format

    :return: Dictionary with language and predictions. Each of them is a list, with
            list as many text as in the request array.
    """
    global language_model
    if language_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    languages = []
    predictions = []
    for text in request.text:
        prediction = language_model.predict(text, k=2)
        languages.append([x.replace("__label__", "").replace("_Latn", "")
                          for x in prediction[0]])
        predictions.append(["{:.8f}".format(y) for y in prediction[1]])

    return {
        "language": languages,
        "predicted": predictions
    }


@app.post("/report/sarcasm")
async def get_sarcasm(request: TextRequest):
    """
    Get sarcasm of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"sarcasm": text}


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
