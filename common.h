#ifndef DISTRIBUTED_BOTNET_COMMON_H
#define DISTRIBUTED_BOTNET_COMMON_H

#include <string>

struct Request {
    std::string url;
};

struct Response {
    int status_code;
    std::string body;
};


#endif //DISTRIBUTED_BOTNET_COMMON_H
