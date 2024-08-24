from models.logs import (
    CPU,
    Logs,
    Process
)
import json
from fastapi import Depends, FastAPI, Body, HTTPException, Path, Query, Request, Response
from fastapi.responses import JSONResponse
from typing import List


app = FastAPI()
app.title = "API for SOPES 2024 course"
app.version = "1.0.0"

logs = Logs()

@app.get("/")
def read_root():
    return {"message": "Welcome to the API for SOPES 2024 course"}

@app.post("/read-module")
def read_module(cpu: CPU):
    logs.add_cpu_data(cpu)
    with open("logs/all-logs.json", "w") as file:
        file.write(json.dumps([cpu.dict() for cpu in logs.get_cpu_data()], indent=4))
        print("Module read successfully")
    return JSONResponse(status_code=200, content={"message": "Module read successfully"})