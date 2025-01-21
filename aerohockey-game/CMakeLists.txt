cmake_minimum_required(VERSION 3.22)
project(game_aerohockey_blockain C)

set(CMAKE_C_STANDARD 17)

add_executable(game_aerohockey_blockain
        aerohockey-game/main.c)

find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

target_link_libraries(game_aerohockey_blockain ${SDL2_LIBRARIES})
