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
