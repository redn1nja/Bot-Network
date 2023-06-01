from fastapi import FastAPI, Request, Form, File, UploadFile
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates
import shutil
import os
import requests

app = FastAPI()
templates = Jinja2Templates(directory="templates")

@app.get("/", response_class=HTMLResponse)
async def get_form(request: Request):
    return templates.TemplateResponse("home.html", {"request": request})

@app.post("/start_attack", response_class=HTMLResponse)
async def start_attack(request: Request, url: str = Form(...), file: UploadFile = File(...)):
    # Process the URL and file as needed
    print(f"URL: {url}")
    requests.post("http://localhost:8080/api/attack", json={"attack" : url})
    file_path = os.path.join("uploads", "configs", file.filename)
    requests.post("http://localhost:8080/is_updating", json={"is_updating" : file_path})
    with open(file_path, "wb") as buffer:
        shutil.copyfileobj(file.file, buffer)
    
    attack_status = "active"
    return templates.TemplateResponse("attack.html", {"request": request, "url": url, "status": attack_status})

@app.get("/attack", response_class=HTMLResponse)
async def attack(request: Request, url: str, status: str):
    return templates.TemplateResponse("attack.html", {"request": request, "url": url, "status": status})

@app.get("/statistics", response_class=HTMLResponse)
async def statistics(request: Request):
    # Retrieve attack statistics
    request_time = "0.0001"
    throughput = "1000"
    ip_status = "active"
    return templates.TemplateResponse("statistics.html", {"request": request, "request_time": request_time, "throughput": throughput, "ip_status": ip_status})

@app.get("/update", response_class=HTMLResponse)
async def show_update_form(request: Request):
    return templates.TemplateResponse("update.html", {"request": request})

@app.post("/update", response_class=HTMLResponse)
async def update(request: Request, file: UploadFile = File(...)):
    file_path = os.path.join("uploads","update", file.filename)
    with open(file_path, "wb") as buffer:
        shutil.copyfileobj(file.file, buffer)
    
    return templates.TemplateResponse("home.html", {"request": request})

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
