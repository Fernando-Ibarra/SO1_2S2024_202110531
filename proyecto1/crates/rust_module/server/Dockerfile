FROM python:3.12.5-alpine3.20

WORKDIR /app

COPY requirements.txt requirements.txt

RUN pip install --no-cache-dir --upgrade -r requirements.txt

COPY . .

EXPOSE 8000

CMD ["fastapi", "run", "main.py", "--port", "8000"]