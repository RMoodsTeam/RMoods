ARG PYTHON_VERSION=3.12.4
FROM python:${PYTHON_VERSION}-slim as base

# Prevents Python from writing pyc files.
ENV PYTHONDONTWRITEBYTECODE=1

# Keeps Python from buffering stdout and stderr to avoid situations where
# the application crashes without emitting any logs due to buffering.
ENV PYTHONUNBUFFERED=1

# Create a virtual environment where the application's dependencies will be installed.
ENV VIRTUAL_ENV=/opt/venv 
RUN python -m venv $VIRTUAL_ENV 
RUN /opt/venv/bin/pip install --upgrade pip

ENV PATH="$VIRTUAL_ENV/bin:$PATH"

# Install the application's dependencies.
COPY . .
RUN pip install -r requirements.txt

# Expose the port that the application listens on.
EXPOSE 8002

# Run the application.
CMD ["python", "src/main.py"]
