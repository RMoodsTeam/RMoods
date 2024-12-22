import os

from fastapi.security.api_key import APIKeyHeader
from fastapi import Security, HTTPException
from starlette.status import HTTP_403_FORBIDDEN
from dotenv import load_dotenv

api_key_header = APIKeyHeader(name="access_token", auto_error=False)
load_dotenv()
API_KEY = os.getenv("NLP_API_KEY")


async def get_api_key(api_key: str = Security(api_key_header)):
    """
    Validates the provided API key.

    Args:
        api_key (str): The API key provided in the request header.

    Returns:
        bool: True if the API key is valid, otherwise raises an HTTPException.

    Raises:
        HTTPException: If the API key is invalid or missing.
    """
    if API_KEY is None:
        raise HTTPException(
            status_code=HTTP_403_FORBIDDEN,
            detail="API KEY is not set in the environment"
        )

    if api_key == API_KEY:
        return True
    else:
        raise HTTPException(
            status_code=HTTP_403_FORBIDDEN, detail="Could not validate API KEY"
        )
