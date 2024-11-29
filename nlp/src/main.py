import os
import string
from contextlib import asynccontextmanager
from typing import List

import fasttext
from fastapi import FastAPI, HTTPException
from keybert import KeyBERT
from langcodes import tag_is_valid, Language
from pydantic import BaseModel
from src.version_checker import update_model_versions
from transformers import AutoTokenizer, AutoModelForSequenceClassification, pipeline

language_model = None
sentiment_tokenizer = None
sentiment_model = None
sarcastic_pipeline = None
spam_pipeline = None
political_pipeline = None
hate_speech_pipeline = None
clickbait_pipeline = None
keyword_model = None


def load_model(model_name: str):
    """
    Load spam models function. It loads the model and tokenizer for the given
    model name.

    :param model_name: Name of the model to load.
    """
    global spam_pipeline, sarcastic_pipeline, political_pipeline, \
        hate_speech_pipeline, clickbait_pipeline, sentiment_model, \
        sentiment_tokenizer, keyword_model, language_model

    model_path = os.path.join("models", model_name, "v1")

    if not os.path.exists(model_path) and model_name != "keywords":
        raise RuntimeError("Model file not found")

    if model_name != "keywords" and model_name != "language":
        try:
            tokenizer = AutoTokenizer.from_pretrained(model_path)
            model = AutoModelForSequenceClassification.from_pretrained(
                model_path)
        except ValueError as e:
            raise RuntimeError(f"Failed to load model: {e}")
    elif model_name == "keywords":
        keyword_model = KeyBERT()
    elif model_name == "language":
        try:
            language_model = fasttext.load_model(model_path + "/model.bin")
        except ValueError as e:
            raise RuntimeError(f"Failed to load model: {e}")

    if model_name == "spam":
        spam_pipeline = pipeline("text-classification", model=model,
                                 tokenizer=tokenizer)
    elif model_name == "sarcasm":
        sarcastic_pipeline = pipeline("text-classification", model=model,
                                      tokenizer=tokenizer)
    elif model_name == "sentiment":
        sentiment_model = model
        sentiment_tokenizer = tokenizer
    elif model_name == "political":
        political_pipeline = pipeline("text-classification", model=model,
                                      tokenizer=tokenizer)
    elif model_name == "hate_speech":
        hate_speech_pipeline = pipeline("text-classification", model=model,
                                        tokenizer=tokenizer)
    elif model_name == "clickbait":
        clickbait_pipeline = pipeline("text-classification", model=model,
                                      tokenizer=tokenizer)


def preprocess_data(input_text):
    """
    Preprocess input text.

    :param input_text: Input text to preprocess.
    :return: Preprocessed text.
    """
    return input_text.lower().translate(str.maketrans('', '', string.punctuation))


def confidence_output(value: float | str):
    """
    Convert confidence value to float.

    :param value: Confidence value.
    :return: Float value of confidence.
    """
    return float(round(value, 2))


@asynccontextmanager
async def lifespan(application: FastAPI):
    """
    Context manager for the lifespan of the application.

    :param application: FastAPI application.
    """
    print("Application is starting.")
    update_model_versions()

    print("Loading language model.")
    load_model("language")

    print("Loading sentiment model.")
    load_model("sentiment")

    print("Loading sarcastic model.")
    load_model("sarcasm")

    print("Loading spam model.")
    load_model("spam")
    #
    print("Loading politics model.")
    load_model("political")

    print("Loadaing hate_speech model.")
    load_model("hate_speech")

    print("Loading clickbait model.")
    load_model("clickbait")

    print("Loading keyword model.")
    load_model("keywords")
    yield
    print("Application is shutting down.")


app = FastAPI(lifespan=lifespan)


class TextRequest(BaseModel):
    """Request model for text"""
    text: List[str]


class AnalysisResult(BaseModel):
    """Result model for text"""
    labels: List[str] = []
    confidences: List[float] = []

    def __init__(self, labels: List[str], confidences: List[float]):
        super().__init__()
        self.labels = labels
        self.confidences = confidences


class Metadata(BaseModel):
    """Metadata model for text"""
    generated_in: float = 0.0

    def __init__(self, generated_in: float):
        super().__init__()
        self.generated_in = generated_in


class TextResponse(BaseModel):
    """Response model for text"""
    kind: str = ""
    metadata: Metadata = Metadata(generated_in=-1.0)
    results: List[AnalysisResult] = []

    def __init__(self, kind: str, metadata: Metadata, results: List[AnalysisResult]):
        super().__init__()
        self.kind = kind
        self.metadata = metadata
        self.results = results

    def json(self):
        """Convert object to json"""
        return {
            "kind": self.kind,
            "metadata": self.metadata,
            "results": self.results
        }


@app.post("/sentiment", response_model=TextResponse)
async def get_sentiment(request: TextRequest):
    """
    Get sentiment of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    if sentiment_model is None or sentiment_tokenizer is None:
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


@app.post("/language", response_model=TextResponse)
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


@app.post("/sarcasm", response_model=TextResponse)
async def get_sarcasm(request: TextRequest):
    """
    Get sarcasm of the text. 0 is not sarcastic, 1 is sarcastic.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    if sarcastic_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        "0": "NOT_SARCASTIC",
        "1": "SARCASTIC"
    }
    results = []

    for text in request.text:
        predict = sarcastic_pipeline(text)
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


@app.post("/keywords", response_model=TextResponse)
async def get_keywords(request: TextRequest):
    """
    Get keywords of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    # Rememeber add author if we wan to use this model
    # How keywords will work with long text?
    if keyword_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        keywords = keyword_model.extract_keywords(text, top_n=5)
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


@app.post("/spam", response_model=TextResponse)
async def get_spam(request: TextRequest):
    """
    Get spam of the text. 0 is not spam, 1 is spam.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    if spam_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    labels = {
        "0": "NOT_SPAM",
        "1": "SPAM"
    }

    results = []
    for text in request.text:
        predict = spam_pipeline(text)
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


@app.post("/politics", response_model=TextResponse)
async def get_politics(request: TextRequest):
    """
    Get politics of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    # Model accuracy may not hold up on pieces of text longer than a tweet.
    # Slice it to smaller pieces if needed?
    if political_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        predict = political_pipeline(text)
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


@app.post("/hate-speech", response_model=TextResponse)
async def get_hate_speech(request: TextRequest):
    """
    Get hate speech of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    # Do zamieszczenia bibliografie z linku
    # https: // huggingface.co / Hate - speech - CNERG / dehatebert - mono - english
    # Pamiętamy
    if hate_speech_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        predict = hate_speech_pipeline(text)
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


@app.post("/clickbait", response_model=TextResponse)
async def get_clickbait(request: TextRequest):
    """
    Get clickbait of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output. Label is set to CLICKBAIT or NOT_CLICKBAIT.
    """
    if clickbait_pipeline is None:
        raise HTTPException(status_code=500, detail="Model not loaded")

    results = []
    for text in request.text:
        predict = clickbait_pipeline(text)
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
    Get troll of the text.

    :param request: Request from server in json format

    :return: Dictionary with model output
    """
    text = request.text[0]
    return {"troll": text}
