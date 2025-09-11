cmake_minimum_required(VERSION 3.16)
project(awq LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

FetchContent_Declare(
    ftxui
    GIT_REPOSITORY https://github.com/ArthurSonzogni/FTXUI.git
    GIT_TAG main
)

FetchContent_MakeAvailable(ftxui)

add_executable(awq main.cpp
        AwqEditor.cpp
        AwqEditor.hpp)
target_link_libraries(awq PRIVATE ftxui::screen ftxui::dom ftxui::component)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    target_compile_definitions(awq PRIVATE FTXUI_MICROSOFT_TERMINAL_FLICKER_WORKAROUND)
endif()