# main.py
from fastapi import FastAPI, Form, Request, HTTPException
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates
from fastapi.staticfiles import StaticFiles
import requests as r

app = FastAPI()
templates = Jinja2Templates(directory="templates")
app.mount("/static", StaticFiles(directory="static"), name="static")

@app.get("/", response_class=HTMLResponse)
async def get_form(request: Request):
    return templates.TemplateResponse("form.html", {"request": request})

@app.post("/submit_url", response_class=HTMLResponse)
async def submit_url(request: Request, url: str = Form(...)):
    print(f"URL: {url}")
    a = r.post("http://localhost:8080/api/attack", json={'attack': url})
    if a.status_code == 200:
        return templates.TemplateResponse("result.html", {"request": request, "message": "Work started"})
    else:
        raise HTTPException(status_code=400, detail="Bad Request")

@app.post("/stop_attack", response_class=HTMLResponse)
async def stop_attack(request: Request):
    a = r.post("http://localhost:8080/api/attack", json={'attack': ""})
    return templates.TemplateResponse("result.html", {"request": request, "message": "Attack stopped"})