#include <iostream>
#include <string>
#include <thread>
#include <vector>
#include <boost/asio.hpp>

// Function to handle a single client connection
void handle_connection(boost::asio::ip::tcp::socket socket)
{
    boost::asio::streambuf request_buf;
    boost::system::error_code ec;
    // Read the request message from the client
    boost::asio::read_until(socket, request_buf, "\n", ec);
    if (ec) {
        std::cerr << "Error receiving request: " << ec.message() << std::endl;
        return;
    }

    std::string request_str = boost::asio::buffer_cast<const char*>(request_buf.data());
    std::cout << "Request: " << request_str;

// Send the response message to the client
    std::string response_str = "200 OK\nHello, world!\n";
    boost::asio::write(socket, boost::asio::buffer(response_str), ec);
    if (ec) {
        std::cerr << "Error sending response: " << ec.message() << std::endl;
        return;
    }
}

int main(int argc, char* argv[])
{
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " <port>" << std::endl;
        return 1;
    }
    try {
        boost::asio::io_service io_service;

        // Bind to the specified port
        boost::asio::ip::tcp::acceptor acceptor(io_service, boost::asio::ip::tcp::endpoint(boost::asio::ip::tcp::v4(), std::stoi(argv[1])));

        std::cout << "Server listening on port " << argv[1] << std::endl;

        // Accept incoming client connections
        while (true) {
            boost::asio::ip::tcp::socket socket(io_service);
            acceptor.accept(socket);
            std::cout << "Accepted connection from " << socket.remote_endpoint().address().to_string() << std::endl;
            std::thread(handle_connection, std::move(socket)).detach();
        }
    }
    catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
    }

    return 0;
}