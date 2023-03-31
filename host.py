import socket
import threading
from client import Client

class Host:
    def __init__(self, known_clients, address, updator) -> None:
        self.known_clients = known_clients
        self.address = address
        self.updator = updator
        
    def request_data_from_clients(self):
        data = []
        for client in self.known_clients:
            client.set_requests()
            data.append((client.get_own_address(), client.send_datum()))
        return data

    def change_address(self, new_addresss):
        self.address = new_addresss
                
    def run(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind((self.address))
            s.listen()
            print(f"Host is running on {self.address}")
            while True:
                conn, addr = s.accept()
                client = Client(address=addr, host_address=self.address)
                self.known_clients.add(client)
                print(f"Connected clients: {len(self.clients)}")
                data = self.request_data_from_clients()
                