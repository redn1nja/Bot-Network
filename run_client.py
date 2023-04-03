from client import Client
import requests

if __name__ == "__main__":
    server_address = ("0.0.0.0", 8000)
    client = Client(server_address)
    client.run()
    # Send IP address and data to the server
    # attack_address = "https://google.com"  # python3 -m http.server
    # num_requests = 5
    # data = client.send_datum(attack_address, num_requests)
    # requests.post(f"http://{server_address[0]}:{server_address[1]}/send_client_data/{client_id}", json=data)
