import src.authorization as auth
import src.globals as globals
import logging

from src.logger_config import logger
from src.base_models import *
from src.models_loading import *
from src.utils import *
from src.version_checker import update_model_versions
from fastapi import FastAPI, HTTPException
from fastapi.params import Depends
from langcodes import tag_is_valid, Language
from contextlib import asynccontextmanager
from fastapi.security.api_key import APIKey


@asynccontextmanager
async def lifespan(application: FastAPI):
    """
    Context manager for the lifespan of the application.

    Args
        application (FastAPI): FastAPI application.
    """
    logger.info("Application is starting")
    logger.info("Checking for model updates")
    try:
        update_model_versions()
        logger.debug("Model versions updated successfully")
    except Exception as e:
        logger.error(f"Failed to update model versions: {e}")
        raise

    models = [
        "language", "sentiment", "sarcasm", "spam",
        "political", "hate_speech", "clickbait", "keywords"
    ]

    for model_name in models:
        logger.info(f"Loading {model_name} model")
        try:
            load_model(model_name)
            logger.debug(f"{model_name.title()} model loaded successfully")
        except Exception as e:
            logger.error(f"Failed to load {model_name} model: {e}")
            raise

    logger.info("All models loaded successfully")
    yield
    logger.info("Application is shutting down")


app = FastAPI(lifespan=lifespan)


@app.post("/sentiment", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_sentiment(request: TextRequest):
    """
    Get sentiment of the text.

    Args:
        request (TextReques): Contains a list of text strings to analyze

    Returns:
        TextResponse: Sentiment analysis results with labels and confidence scores.
    """
    if globals.sentiment_model is None or globals.sentiment_tokenizer is None:
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
        tokenized_text = globals.sentiment_tokenizer([preprocess_data(text)],
                                                     padding=True, truncation=True,
                                                     max_length=128,
                                                     return_tensors="pt")
        output = globals.sentiment_model(**tokenized_text)
        probs = output.logits.softmax(dim=-1).tolist()[0]
        confidence = max(probs)
        prediction = probs.index(confidence)
        result = AnalysisResult(
            labels=[labels[prediction]],
            confidences=[confidence_output(confidence)]
        )
        results.append(result)

    return TextResponse(
        kind="sentiment",
        metadata=Metadata(generated_in=0.0),
        results=results
    ).json()


@app.post("/language", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_language(request: TextRequest):
    """
    Get language of the texts from request.

    Args:
        request (TextReques): Contains a list of text strings to analyze

    Returns:
        TextResponse: Language detection results with labels and confidence scores.
    """
    if globals.language_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        languages = []
        prediction = globals.language_model.predict(text, k=2)

        for predicted_lang in prediction[0]:
            lang_tag = predicted_lang.rsplit("_")[-2]
            if tag_is_valid(lang_tag):
                lang_name = Language.get(lang_tag).display_name("en")
                languages.append(lang_name)
            else:
                languages.append(lang_tag)

        result = AnalysisResult(
            labels=languages,
            confidences=[confidence_output(y) for y in prediction[1]]
        )
        results.append(result)

    return TextResponse(
        kind="language",
        metadata=Metadata(generated_in=0.0),
        results=results
    ).json()


@app.post("/sarcasm", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_sarcasm(request: TextRequest):
    """
    Get sarcasm of the text. 0 is not sarcastic, 1 is sarcastic.

    Args:
        request (TextReques): Contains a list of text strings to analyze

    Returns:
        TextResponse: Sarcasm detection results with labels and confidence scores.
    """
    if globals.sarcastic_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        "0": "NOT_SARCASTIC",
        "1": "SARCASTIC"
    }
    results = []

    for text in request.text:
        predict = globals.sarcastic_pipeline(text)
        result = AnalysisResult(
            labels=[labels[predict[0]["label"].replace("LABEL_", "")]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        results.append(result)

    return TextResponse(
        kind="sarcasm",
        metadata=Metadata(generated_in=0.0),
        results=results
    ).json()


@app.post("/keywords", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_keywords(request: TextRequest):
    """
    Get keywords of the text.

    Args:
        request (TextReques): Contains a list of text strings to analyze

    Returns:
        TextResponse: Keyword extraction results with labels and relevance scores.
    """
    # Rememeber add author if we wan to use this model
    # How keywords will work with long text?
    if globals.keyword_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        keywords = globals.keyword_model.extract_keywords(text, top_n=5)
        result = AnalysisResult(
            labels=[kw[0] for kw in keywords],
            confidences=[kw[1] for kw in keywords]
        )
        results.append(result)

    return TextResponse(
        kind="keywords",
        metadata=Metadata(generated_in=0.0),
        results=results
    ).json()


@app.post("/spam", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_spam(request: TextRequest):
    """
    Get spam of the text. 0 is not spam, 1 is spam.

    Args:
        request (TextReques): Contains a list of text strings to analyze

    Returns:
        TextResponse: Spam detection results with labels and confidence scores.
    """
    if globals.spam_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        "0": "NOT_SPAM",
        "1": "SPAM"
    }

    results = []
    for text in request.text:
        predict = globals.spam_pipeline(text)
        result = AnalysisResult(
            labels=[labels[predict[0]["label"].replace("LABEL_", "")]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        results.append(result)

    return TextResponse(
        kind="spam",
        metadata=Metadata(generated_in=0.0),
        results=results
    ).json()


@app.post("/politics", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_politics(request: TextRequest):
    """
    Get politics of the text.

    Args:
        request (TextReques): Contains a list of text strings to analyze.

    Returns:
        TextResponse: Political content detection results with labels and confidence scores.
    """
    # Model accuracy may not hold up on pieces of text longer than a tweet.
    # Slice it to smaller pieces if needed?
    if globals.political_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        predict = globals.political_pipeline(text)
        result = AnalysisResult(
            labels=[predict[0]["label"]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        results.append(result)

    return TextResponse(
        kind="politics",
        metadata=Metadata(generated_in=0.0),
        results=results
    ).json()


@app.post("/hate-speech", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_hate_speech(request: TextRequest):
    """
    Get hate speech of the text.

    Args:
        request (TextReques): Contains a list of text strings to analyze.

    Returns:
        TextResponse: Hate speech detection results with labels and confidence scores.
    """
    # Do zamieszczenia bibliografie z linku
    # https: // huggingface.co / Hate - speech - CNERG / dehatebert - mono - english
    # Pamiętamy
    if globals.hate_speech_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        predict = globals.hate_speech_pipeline(text)
        result = AnalysisResult(
            labels=[predict[0]["label"]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        results.append(result)

    return TextResponse(
        kind="hate_speech",
        metadata=Metadata(generated_in=0.0),
        results=results
    ).json()


@app.post("/clickbait", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_clickbait(request: TextRequest):
    """
    Get clickbait of the text.

    Args:
        request (TextReques): Contains a list of text strings to analyze.

    Returns:
        TextResponse: Clickbait detection results with labels and confidence scores.
    """
    if globals.clickbait_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        predict = globals.clickbait_pipeline(text)
        result = AnalysisResult(
            labels=[predict[0]["label"]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        results.append(result)

    return TextResponse(
        kind="clickbait",
        metadata=Metadata(generated_in=0.0),
        result=results
    ).json()


# TODO: Implement troll model
@app.post("/troll")
async def get_troll(request: TextRequest):
    """
    Detect troll content in the provided text(s).

    Args:
        request (TextRequest): Contains a list of text strings to analyze.

    Returns:
        dict: Troll detection results.
    """
    text = request.text[0]
    return {"troll": text}
