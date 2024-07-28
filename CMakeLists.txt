cmake_minimum_required(VERSION 3.23)
project(IsOddUser LANGUAGES C)

if(USER_FIND_PACKAGE)
    find_package(IsOdd REQUIRED)
else()
    add_subdirectory(is_odd)
endif()

add_executable(IsOddUser main.c)
target_link_libraries(IsOddUser PRIVATE IsOdd::is_odd)
