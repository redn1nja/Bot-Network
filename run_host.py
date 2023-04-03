
from host import Host

if __name__ == "__main__":
    server_address = ("0.0.0.0", 8000)
    host = Host(server_address)
    host.run()
    # Register a new client with its IP address
    # client_id = host.register_client("127.0.1.1")

    # Send the IP address to the client
    # host.send_ip(client_id, "https://google.com")  # python3 -m http.server

    # Get client data from the server
    # data = host.get_client_data(client_id)
    # print(data)
