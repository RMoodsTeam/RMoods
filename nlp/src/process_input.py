import inspect
from typing import List, Callable

from src.base_models import AnalysisResult
from src.logger_config import logger


def process_inputs(f: Callable[[str], AnalysisResult], texts: List[str]) -> List[AnalysisResult]:
    """Process input texts."""
    caller_name = inspect.stack()[1].function
    results = []
    for (i, text) in enumerate(texts):
        logger.info(f"{caller_name}: Processing text {i + 1}/{len(texts)} - {(i + 1) / len(texts) * 100:.2f}%")
        result = f(text)
        results.append(result)

    return results
