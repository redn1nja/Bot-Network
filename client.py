import socket
import requests


class Client:
    ID = 0

    def __init__(self, address=None, requests=None) -> None:
        self.__id = ID
        ID += 1
        self.__attack_address = address
        self.__own_address = socket.gethostbyname(socket.gethostname())
        self.__requests = requests
        self.__data = {}

    def set_address(self, address):
        self.__attack_address = address

    def set_requests(self, no):
        self.__requests = no

    def get_own_address(self):
        return self.__own_address

    def __send_request(self):
        r = requests.get(self.__attack_address)
        return r.return_code, r.elapsed

    def start_requesting(self):
        while (self.__requests):
            status, time = self.__send_request()
            if status not in self.__data.keys:
                self.__data[status] = [0, 0]
            self.__data[status][0] += 1
            self.__data[status][1] += time
            self.__requests -= 1

    def send_datum(self):
        data = self.__data
        self.__data = {}
        return data
