ARG PYTHON_VERSION=3.12.4
FROM python:${PYTHON_VERSION}-slim as base

# Prevents Python from writing pyc files.
ENV PYTHONDONTWRITEBYTECODE=1

# Keeps Python from buffering stdout and stderr to avoid situations where
# the application crashes without emitting any logs due to buffering.
ENV PYTHONUNBUFFERED=1

# Install the application's dependencies.
RUN apt-get update && apt-get install build-essential -y

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy the application's source code.
COPY . .

# Expose the port that the application listens on.
EXPOSE 8002

# Run the application.
CMD ["uvicorn", "src.main:app", "--host", "0.0.0.0", "--port", "8002"]
