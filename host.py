import socket
import threading
from client import Client

class Host:
    def __init__(self, known_clients, address, updator) -> None:
        self.known_clients = known_clients
        self.address = address
        self.updator = updator
        
    def handle_connection(self, connection):
        with connection:
            while True:
                data = connection.recv(1024)
                if not data:
                    break
                connection.sendall(data)
                
    def request_data_from_clients(self):
        return [client.get_data() for client in self.clients]

    def change_address(self, new_addresss):
        self.address = new_addresss
                
    def run(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind(('localhost', self.port))
            s.listen()
            while True:
                conn, addr = s.accept()
                if self.running:
                    threading.Thread(target=self.handle_connection, args=(conn, addr)).start()
                else:
                    conn.close()

                client = Client(conn, addr)
                self.known_clients.add(client)
                print(f"Connected clients: {len(self.clients)}")