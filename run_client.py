from client import Client

if __name__ == "__main__":
    server_address = ("0.0.0.0", 8000)
    client = Client(":".join([str(x) for x in server_address]))
    print("client started")
    client.run()
