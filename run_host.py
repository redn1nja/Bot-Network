from host import Host


def print_help():
    print("Main commands:")
    print("1. To set attack info, type 'set'")
    print("2. To view this message, type 'h'")
    print("3. To start the attack, type 'start'")
    print("4. To stop the attack, type 'stop'")


def set_attack_info():
    hoster.set_attack_address()
    hoster.set_requests_amount()
    print("Attack info set successfully")


def start_attack_wrapper():
    hoster.start_attack()
    print("Attack turned on")


def stop_attack_wrapper():
    hoster.stop_attack()
    print("Attack turned off")


if __name__ == "__main__":
    server_address = ("0.0.0.0", 8000)
    hoster = Host(":".join([str(x) for x in server_address]))

    INP_OPTIONS = {
        "set": set_attack_info,
        "h": print_help,
        "start": start_attack_wrapper,
        "stop": stop_attack_wrapper
    }

    print("Welcome to the DDoS attack simulator!")
    print("Type 'h' to see the list of commands")
    print("Enter command")

    while True:
        inp = input("> ")
        if inp in INP_OPTIONS:
            INP_OPTIONS[inp]()
        else:
            print("Unknown command. Type 'h' to see the list of commands")
            continue
