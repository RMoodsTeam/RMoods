import time

from fastapi import FastAPI
from src.version_checker import update_model_versions
from pydantic import BaseModel
from typing import List
app = FastAPI()

class TextRequest(BaseModel):
    text: List[str]


@app.post("/report/sentiment")
async def get_sentiment(request: TextRequest):
    text = request.text[0]
    return {"sentiment": text}

@app.post("/report/language")
async def get_language(request: TextRequest):
    text = request.text[0]
    return {"language": text}

@app.post("/report/sarcasm")
async def get_sarcasm(request: TextRequest):
    text = request.text[0]
    return {"sarcasm": text}

@app.post("/report/keywords")
async def get_keywords(request: TextRequest):
    text = request.text[0]
    return {"keywords": text}

@app.post("/report/spam")
async def get_spam(request: TextRequest):
    text = request.text[0]
    return {"spam": text}


@app.post("/report/politics")
async def get_politics(request: TextRequest):
    text = request.text[0]
    return {"politics": text}

@app.post("/report/hate-speach")
async def get_hate_speech(request: TextRequest):
    text = request.text[0]
    return {"hate-speech": text}

@app.post("/report/clickbait")
async def get_clickbait(request: TextRequest):
    text = request.text[0]
    return {"clickbait": text}

@app.post("/report/troll")
async def get_troll(request: TextRequest):
    text = request.text[0]
    return {"troll": text}

if __name__ == '__main__':
    if update_model_versions():
        import uvicorn
        uvicorn.run(app)
