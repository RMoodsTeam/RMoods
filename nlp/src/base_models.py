from pydantic import BaseModel
from typing import List


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

class TextResponse(BaseModel):
    """Response model for text"""
    kind: str = ""
    generatedIn: float = -1.0
    results: List[AnalysisResult] = []

    def __init__(self, kind: str, generatedIn: float, results: List[AnalysisResult]):
        super().__init__()
        self.kind = kind
        self.generatedIn = generatedIn
        self.results = results

    def json(self):
        """Convert object to json"""
        return {
            "kind": self.kind,
            "generatedIn": self.generatedIn,
            "results": self.results
        }


class ModelSupportedLanguages(BaseModel):
    """Response model for used language"""
    sentiment: List[str]
    language: List[str]
    sarcasm: List[str]
    keywords: List[str]
    spam: List[str]
    political: List[str]
    hateSpeech: List[str]
    clickbait: List[str]
    llm: List[str]

    def json(self):
        return {
            "sentiment": self.sentiment,
            "language": self.language,
            "sarcasm": self.sarcasm,
            "keywords": self.keywords,
            "spam": self.spam,
            "political": self.political,
            "hateSpeech": self.hateSpeech,
            "clickbait": self.clickbait,
            "llm": self.llm
        }
