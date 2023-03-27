#include <iostream>
#include <chrono>
#include <thread>
#include <boost/asio.hpp>
#include "common.h"

// Function to send a single request to the server and receive the response
Response send_request(boost::asio::io_service& io_service, const std::string& server_host, const std::string& server_port, const Request& request)
{
    Response response;

    // Resolve the server endpoint
    boost::asio::ip::tcp::resolver resolver(io_service);
    boost::asio::ip::tcp::resolver::query query(server_host, server_port);
    boost::asio::ip::tcp::resolver::iterator endpoint_iterator = resolver.resolve(query);

    // Establish a connection to the server
    boost::asio::ip::tcp::socket socket(io_service);
    boost::system::error_code ec;
    boost::asio::connect(socket, endpoint_iterator, ec);
    if (ec) {
        std::cerr << "Error connecting to server: " << ec.message() << std::endl;
        return response;
    }

    // Send the request message to the server
    std::string request_str = request.url + "\n";
    boost::asio::write(socket, boost::asio::buffer(request_str), ec);
    if (ec) {
        std::cerr << "Error sending request: " << ec.message() << std::endl;
        return response;
    }

    // Read the response message from the server
    boost::asio::streambuf response_buf;
    boost::asio::read_until(socket, response_buf, "\n", ec);
    if (ec) {
        std::cerr << "Error receiving response: " << ec.message() << std::endl;
        return response;
    }

    std::string response_str = boost::asio::buffer_cast<const char*>(response_buf.data());
    std::cout << "Response: " << response_str;

    // Parse the response message
    response.status_code = std::stoi(response_str.substr(0, 3));
    response.body = response_str.substr(4, response_str.size() - 5); // Remove status code and newline character

    return response;
}

int main(int argc, char* argv[])
{
    if (argc != 5) {
        std::cerr << "Usage: " << argv[0] << " <server_host> <server_port> <num_requests>" << std::endl;
        return 1;
    }

    try {
        boost::asio::io_service io_service;

        std::string server_host = argv[1];
        std::string server_port = argv[2];
        int num_requests = std::stoi(argv[3]);

        // Send requests in parallel using multiple threads
        std::vector<std::thread> threads;
        for (int i = 0; i < num_requests; i++) {
            Request request;
            request.url = "/test"; // TODO: Replace with actual URL

            threads.emplace_back([&io_service, server_host, server_port, request]() {
                send_request(io_service, server_host, server_port, request);
            });
        }

        for (auto& t : threads) {
            t.join();
        }
    }
    catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
    }

    return 0;
}
