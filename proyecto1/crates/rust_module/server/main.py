from models.logs import (
    CPU,
    Logs,
    Process
)
import json
from fastapi import Depends, FastAPI, Body, HTTPException, Path, Query, Request, Response
from fastapi.responses import JSONResponse
from typing import List
import matplotlib.pyplot as plt


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


@app.post("/make_graphs")
def make_graphs():
    # read logs from file and create plot graphs for total_ram, free_ram and ram_in_use
    total_ram = []
    free_ram = []
    ram_in_use = []
    time = []
    with open("logs/all-logs.json", "r") as file:
        data = json.load(file)
        # make a for with index
        time_value = 0
        for cpu in data:
            time_value += 1
            total_ram.append(cpu["total_ram"])
            free_ram.append(cpu["free_ram"])
            ram_in_use.append(cpu["ram_in_use"])
            time.append(time_value)
                    
        plt.plot(time, total_ram, label="Total RAM")
        plt.plot(time, free_ram, label="Free RAM")
        plt.plot(time, ram_in_use, label="RAM in use")
        plt.xlabel("Time")
        plt.ylabel("RAM")
        plt.legend()
        plt.savefig("graphs/ram.png")
        plt.close()
    return JSONResponse(status_code=200, content={"message": "Graphs created successfully"})