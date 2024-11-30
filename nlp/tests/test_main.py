import pytest
from httpx import AsyncClient, ASGITransport
from src.main import app

INVALID_API_KEY = "invalid"
@pytest.mark.asyncio
@pytest.mark.parametrize("endpoint", [
    "/sentiment",
    "/language",
    "/sarcasm",
    "/keywords",
    "/spam",
    "/politics",
    "/hate-speech",
    "/clickbait",
])
async def test_bad_responses(endpoint):
    async with AsyncClient(transport=ASGITransport(app=app),
                           base_url="http://test") as ac:
        response = await ac.post(endpoint, json={"text": ["I love this!"]},
                                 headers={"Authorization": f"Bearer {INVALID_API_KEY}"})
        assert response.status_code == 403
        assert "detail" in response.json()
        assert response.json()["detail"] == "Could not validate API KEY"
