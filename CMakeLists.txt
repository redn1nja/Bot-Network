cmake_minimum_required(VERSION 3.15)

project(distributed-botnet C CXX) # project(${PROJECT_NAME} C CXX ASM)

set(CMAKE_CXX_STANDARD 17)

set(WARNINGS_AS_ERRORS OFF)

add_executable(host host.cpp common.h)
add_executable(client client.cpp common.h)


find_package(Boost 1.71.0 COMPONENTS program_options system REQUIRED)
target_include_directories(host PRIVATE ${Boost_INCLUDE_DIR})
target_include_directories(client PRIVATE ${Boost_INCLUDE_DIR})
target_link_libraries(host Boost::program_options Boost::system)
target_link_libraries(client Boost::program_options Boost::system)

INSTALL(PROGRAMS
        $<TARGET_FILE:host>	# ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}
        DESTINATION bin)

INSTALL(PROGRAMS
        $<TARGET_FILE:client>	# ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}
        DESTINATION bin)

set(ALL_TARGETS host)
set(ALL_TARGETS client)

find_package(Threads REQUIRED)
target_link_libraries(host Threads::Threads)
target_link_libraries(client Threads::Threads)