from models.cpu import (
    CPU, Process
)
from typing import List

class Logs:
    def __init__(self):
        self.cpu_data: List[CPU] = []
        
    def add_cpu_data(self, cpu: CPU):
        self.cpu_data.append(cpu)
        
    def get_cpu_data(self):
        return self.cpu_data