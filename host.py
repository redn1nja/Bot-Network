import socket
import requests
from client import Client
from typing import List, Tuple

class Host:
    def __init__(self, server_address: Tuple[str, int]) -> None:
        self.server_address = server_address
        self.attack_address = self.get_attack_adress()
        self.requests_amount = self.get_requests_amount()
    # def register_client(self, client_address: str) -> int:
    #     r = requests.post(f"http://{self.server_address[0]}:{self.server_address[1]}/register_client", json={"client_address": client_address})
    #     return r.json()["client_id"]

    def get_attack_adress(self) -> str:
        return input("Enter the attack address: ")
    
    def get_requests_amount(self) -> int:
        return int(input("Enter the amount of requests: "))

    def send_attack_info(self) -> None:
        attack_info = {"attack_address": self.attack_address, "requests_amount": self.requests_amount}
        requests.post(f"http://{self.server_address[0]}:{self.server_address[1]}/set_attack_info", json=attack_info)
    
    def get_clients_stats(self) -> None:
        r = requests.get(f"http://{self.server_address[0]}:{self.server_address[1]}/get_clients_stats")

    def run(self):
        self.send_attack_info()
        while True:
            continue
        # while True:
        #     self.get_clients_stats()







    # def send_ip(self, client_id: int, ip: str) -> None:
    #     requests.post(f"http://{self.server_address[0]}:{self.server_address[1]}/send_ip/{client_id}", json={"ip": ip})

    # def get_client_data(self, client_id: int):
    #     r = requests.get(f"http://{self.server_address[0]}:{self.server_address[1]}/get_client_data/{client_id}")
    #     return r.json()["data"]
