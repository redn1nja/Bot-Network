from host import Host
from client import Client

host_address = ('localhost', 1234)

client1 = Client(host_address=host_address)
client1.set_address(host_address)
client1.set_requests()

client2 = Client(host_address=host_address)
client2.set_address(host_address)
client2.set_requests()

known_clients = set([client1, client2])

host = Host(known_clients, host_address, None)
data = host.request_data_from_clients()

print(data)