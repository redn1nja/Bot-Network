from fastapi import FastAPI
from pydantic import BaseModel
from typing import Dict, Any

app = FastAPI()

class ClientData(BaseModel):
    client_address: str

class IPAddress(BaseModel):
    ip: str

clients = {}
client_ips = {}
clients_stats = []
attack_info_stored = {"attack_address": "", "requests_amount": 0}

@app.post("/set_attack_info")
async def set_attack_info(attack_info: Dict[str, Any]):
    global attack_info_stored
    attack_info_stored = attack_info

@app.get("/get_attack_info")
async def get_attack_info():
    return attack_info_stored


@app.get("/get_clients_stats")
async def get_clients_stats():
    if clients_stats:
        return {"clients_stats": clients_stats}
    else:
        return {"detail": "No clients stats"}

@app.post("/set_clients_stats")
async def set_clients_stats(client_stats: Dict[str, Any]):
    clients_stats.append(client_stats)



if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
