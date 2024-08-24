from pydantic import BaseModel
class Process(BaseModel):
    pid: int
    name: str
    cpu_usage: int
    command_line: str
    id_container: str
    rss: int
    vsz: int
    mem_usage: float