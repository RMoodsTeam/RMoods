import src.authorization as auth
import src.globals as gl
import time

from contextlib import asynccontextmanager
from fastapi import FastAPI, HTTPException
from fastapi.params import Depends

from docs.conf import language
from src.base_models import *
from src.logger_config import logger
from src.models_loading import *
from src.process_input import process_inputs
from src.utils import *
from src.version_checker import update_model_versions


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
        # update_model_versions()
        logger.debug("Model versions updated successfully")
    except Exception as e:
        logger.error(f"Failed to update model versions: {e}")
        raise

    models = [
        "language", "sentiment", "llm", "sarcasm", "spam",
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
    if gl.language_model is None:
        raise HTTPException(status_code=500, detail="Language model not loaded")

    if (gl.sentiment_model_english is None or gl.sentiment_tokenizer is None
            or gl.sentiment_model_polish is None):
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        0: "Sadness",
        1: "Joy",
        2: "Love",
        3: "Anger",
        4: "Fear",
        5: "Surprise"
    }

    def process_fn(text):
        text = preprocess_data(text)
        lang_name = detect_language(text)
        if lang_name == "Polish":
            predict = gl.sentiment_model_polish(text)
            result = AnalysisResult(
                labels=[predict[0]["label"]],
                confidences=[confidence_output(predict[0]["score"])]
            )
        else:
            tokenized_text = gl.sentiment_tokenizer([preprocess_data(text)],
                                                    padding=True, truncation=True,
                                                    max_length=128,
                                                    return_tensors="pt")
            output = gl.sentiment_model_english(**tokenized_text)
            probs = output.logits.softmax(dim=-1).tolist()[0]
            confidence = max(probs)
            prediction = probs.index(confidence)
            result = AnalysisResult(
                labels=[labels[prediction]],
                confidences=[confidence_output(confidence)]
            )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="sentiment",
        generatedIn=generation_time,
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
    if gl.language_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    def process_fn(text):
        languages = []
        text = preprocess_data(text)
        prediction = gl.language_model.predict(text, k=2)

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
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="language",
        generatedIn=generation_time,
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
    if gl.sarcastic_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        "0": "NOT_SARCASTIC",
        "1": "SARCASTIC"
    }

    def process_fn(text):
        text = preprocess_data(text)
        predict = gl.sarcastic_pipeline(text)
        result = AnalysisResult(
            labels=[labels[predict[0]["label"].replace("LABEL_", "")]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="sarcasm",
        generatedIn=generation_time,
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
    # Remember add author if we want to use this model
    # How keywords will work with long text?
    if gl.keyword_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    def process_fn(text):
        text = preprocess_data(text)
        keywords = gl.keyword_model.extract_keywords(text, top_n=5)
        result = AnalysisResult(
            labels=[kw[0] for kw in keywords],
            confidences=[kw[1] for kw in keywords]
        )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="keywords",
        generatedIn=generation_time,
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
    if gl.spam_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        "0": "NOT_SPAM",
        "1": "SPAM"
    }

    def process_fn(text):
        text = preprocess_data(text)
        predict = gl.spam_pipeline(text)
        result = AnalysisResult(
            labels=[labels[predict[0]["label"].replace("LABEL_", "")]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="spam",
        generatedIn=generation_time,
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
    if gl.political_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    def process_fn(text):
        text = preprocess_data(text)
        predict = gl.political_pipeline(text)
        result = AnalysisResult(
            labels=[predict[0]["label"]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="politics",
        generatedIn=generation_time,
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
    if gl.language_model is None:
        raise HTTPException(status_code=500, detail="Language model not loaded")

    if (gl.hate_speech_english_pipeline is None or
            gl.hate_speech_polish_pipeline is None):
        raise HTTPException(status_code=500, detail="Model not loaded")

    def process_fn(text):
        text = preprocess_data(text)
        lang_name = detect_language(text)
        if lang_name == "Polish":
            predict = gl.hate_speech_polish_pipeline(text)
        else:
            predict = gl.hate_speech_english_pipeline(text)

        result = AnalysisResult(
            labels=[predict[0]["label"]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="hateSpeech",
        generatedIn=generation_time,
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
    if gl.clickbait_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    def process_fn(text):
        text = preprocess_data(text)
        predict = gl.clickbait_pipeline(text)
        result = AnalysisResult(
            labels=[predict[0]["label"]],
            confidences=[confidence_output(predict[0]["score"])]
        )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="clickbait",
        generatedIn=generation_time,
        results=results
    ).json()


@app.post("/llm", response_model=TextResponse,
          dependencies=[Depends(auth.get_api_key)])
async def get_llm(request: TextRequest):
    """
    Detect if the text is written by a AI bot or by human.

    Args:
        request (TextRequest): Contains a list of text strings to analyze.

    Returns:
        dict: Llm detection results.
    """
    if (gl.llm_pipeline_english is None):
        raise HTTPException(status_code=500, detail="Model not loaded")

    def process_fn(text):
        text = preprocess_data(text)
        prediction = gl.llm_pipeline_english(text)
        confidence = prediction[0]["score"]
        if confidence > 0.5:
            label = "llm"
            confidence = confidence_output(confidence)
        else:
            label = "human"
            confidence = confidence_output(1 - confidence)

        result = AnalysisResult(
            labels=[label],
            confidences=[confidence]
        )
        return result

    results, generation_time = measure_execution_time(
        lambda: process_inputs(process_fn, request.text))

    return TextResponse(
        kind="llm",
        generatedIn=generation_time,
        results=results
    ).json()


@app.post("/info/langs", response_model=ModelSupportedLanguages,
          dependencies=[Depends(auth.get_api_key)])
async def get_languages():
    """
    Get languages used in the application.

    Returns:
        ModelSupportedLanguages: Languages used in the application.
    """
    with open("version_models.json", "r") as file:
        data = json.load(file)

    return ModelSupportedLanguages(
        sentiment=data.get("sentiment", []),
        language=data.get("language", []),
        sarcasm=data.get("sarcasm", []),
        keywords=data.get("keywords", []),
        spam=data.get("spam", []),
        political=data.get("political", []),
        hateSpeech=data.get("hate_speech", []),
        clickbait=data.get("clickbait", []),
        llm=data.get("llm", [])
    ).json()
