cmake_minimum_required(VERSION 3.16)
project(cengine C)

# Always generate compile_commands.json this is for zed ide
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_C_STANDARD 11)

include(FetchContent)

# ========== GLFW ==========
FetchContent_Declare(
    glfw
    GIT_REPOSITORY https://github.com/glfw/glfw.git
    GIT_TAG latest
)
FetchContent_MakeAvailable(glfw)

# ========== GLAD ==========
add_library(glad
    external/glad/src/glad.c   # or glad.c if that’s the name
)

target_include_directories(glad
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/external/glad/include
)

add_library(cengine STATIC src/main.c)

# Link: glad first, then glfw
target_link_libraries(cengine
    PRIVATE
        glad
        glfw
)

# Needed on Linux
if(UNIX AND NOT APPLE)
    target_link_libraries(cengine PRIVATE dl pthread)
endif()
