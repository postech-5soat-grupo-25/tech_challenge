FROM python:3.11-slim-buster

ENV PYTHONUNBUFFERED=1
ENV SECRET_KEY=13521685416516

RUN apt-get update && apt-get install -y libpq-dev gcc

RUN pip install poetry
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    htop \
    tzdata \
    && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY pyproject.toml poetry.lock gunicorn_config.py /app/

RUN poetry config virtualenvs.create false && poetry install

ADD . /app/

EXPOSE 9000

USER root

CMD ["gunicorn", "-c", "gunicorn_config.py", "payment_api.wsgi:application"]
