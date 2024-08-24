FROM python:3.12.5-alpine3.20

WORKDIR /app

COPY requirements.txt requirements.txt

RUN pip install -r requirements.txt

COPY . .

CMD ["fastapi", "dev", "main.py"]