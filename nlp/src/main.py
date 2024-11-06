from fastapi import FastAPI
from src.version_checker import update_model_versions
from pydantic import BaseModel
from typing import List
from contextlib import asynccontextmanager


@asynccontextmanager
async def lifespan(app: FastAPI):
    """
    Context manager for the lifespan of the application.
    :param app: FastAPI application.
    """
    print("Application is starting.")
    update_model_versions()
    yield
    print("Application is shutting down.")


app = FastAPI(lifespan=lifespan)

class TextRequest(BaseModel):
    """
    Request model for text
    """
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

@app.post("/report/language")
async def get_language(request: TextRequest):
    """
    Get language of the text.
    :param request: Request from server in json format
    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"language": text}

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
