from models.process import Process
from typing import List

from pydantic import BaseModel

class CPU(BaseModel):
    total_ram: int
    free_ram: int
    ram_in_use: int
    processes: List[Process]
    time: str