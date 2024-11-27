import pytest
from httpx import AsyncClient, ASGITransport
from src.main import app


@pytest.mark.asyncio
@pytest.mark.parametrize("endpoint, expected_response", [
    ("/sentiment", {"sentiment": "I love this!"}),
    ("/language", {"language": "I love this!"}),
    ("/sarcasm", {"sarcasm": "I love this!"}),
    ("/keywords", {"keywords": "I love this!"}),
    ("/spam", {"spam": "I love this!"}),
    ("/politics", {"politics": "I love this!"}),
    ("/hate-speach", {"hate-speach": "I love this!"}),
    ("/clickbait", {"clickbait": "I love this!"}),
    ("/troll", {"troll": "I love this!"}),
])
async def test_bad_responses(endpoint, expected_response):
    async with AsyncClient(transport=ASGITransport(app=app),
                           base_url="http://test") as ac:
        response = await ac.post(endpoint, json={"invalid_key": ["I love this!"]})
        assert response.status_code == 422
        assert "detail" in response.json()
        assert response.json()["detail"][0]["msg"] == "Field required"
