import socket
import requests
import threading

class Listener (threading.Thread):
    def __init__(self, address):
        self.address = address
        threading.Thread.__init__(self)
    def run(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind((self.address, 5432))
            s.listen(2)
            while True:
                conn, addr = s.accept()
                data = conn.recv(1024).decode("ascii")
                if not data:
                    break
                print(data)


class Client:
    ID = 0
    def __init__(self, address=None, requests=None) -> None:
        self.__id = Client.ID
        Client.ID += 1
        self.__attack_address = address
        self.__own_address = socket.gethostbyname(socket.gethostname())
        print(self.get_own_address())
        self.l = Listener(self.__own_address)
        self.l.start()
        self.__requests = requests
        self.__data = {}

    def set_address(self, address):
        self.__attack_address = address

    def set_requests(self):
        self.__requests = requests.get(self.__own_address, "n").json["n"]

    def get_own_address(self):
        return self.__own_address

    def __send_request(self):
        r = requests.get(self.__attack_address)
        return r.status_code, r.elapsed

    def start_requesting(self):
        while (self.__requests):
            status, time = self.__send_request()
            print(status, time)
            if status not in self.__data.keys():
                self.__data[status] = [0, 0]
            self.__data[status][0] += 1
            self.__data[status][1] += int(str(time).split(".")[-1])
            self.__requests -= 1

    def send_datum(self):
        data = self.__data
        self.__data = {}
        return data



if __name__ == "__main__":
    c = Client(requests=1)
    c.set_address("http://0.0.0.0:8000") #python3 -m http.server
    # c.start_requesting()
    # data = c.send_datum()
    c.l.join()
    # print(data)