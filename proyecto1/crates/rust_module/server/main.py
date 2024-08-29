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
    total_ram = []
    free_ram = []
    ram_in_use = []
    time = []
    with open("logs/all-logs.json", "r") as file:
        data = json.load(file)
        for cpu in data:
            print(cpu["total_ram"])
            total_ram.append(cpu["total_ram"])
            print(cpu["free_ram"])
            free_ram.append(cpu["free_ram"])
            print(cpu["ram_in_use"])
            ram_in_use.append(cpu["ram_in_use"])
            time.append(cpu["time"])
                    
        plt.plot(time, total_ram, label="Total RAM")
        plt.plot(time, free_ram, label="Free RAM")
        plt.plot(time, ram_in_use, label="RAM in use")
        plt.xlabel("Time")
        plt.ylabel("RAM")
        plt.legend("RAM usage")
        plt.grid()
        plt.savefig("graphs/ram.png")
        plt.close()
    return JSONResponse(status_code=200, content={"message": "Graphs created successfully"})


@app.post("/make_process_graphs")
def make_process_graphs():
    with open("logs/all-logs.json", "r") as file:
        data = json.load(file)
        size = len(data)
        cols = 3
        rows = ( size + cols - 1 ) // cols
        fig, ax = plt.subplots( rows, cols, figsize=(10, 5 * rows))
        cpu_values = []
        ids = []
        colors = ["blue", "red", "green", "yellow", "purple", "orange", "pink", "brown", "black", "gray"]
        for cpu in data:
            cpu_array = []
            id_container = []
            for process in cpu["processes"]:
                cpu_array.append(process["cpu_usage"])
                id_container.append(process["id_container"][:3])
            cpu_values.append(cpu_array)
            ids.append(id_container)

        for i, ax in enumerate(ax.flat):
            if i < size:
                ax.bar(ids[i], cpu_values[i], color=colors[i])
                ax.set_title(f"Time: {data[i]['time']}")
                ax.set_ylabel("CPU Usage (%)")
                ax.set_xlabel("ID Container")
                ax.grid()
            else:
                ax.axis("off")
        plt.tight_layout()
        plt.savefig("graphs/cpu.png")
    return JSONResponse(status_code=200, content={"message": "Graphs created successfully"})