cmake_minimum_required(VERSION 3.8)
project(Schach)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

include_directories(includes)


if( WIN32 )
    add_compile_options(/Wall /wd4820 /wd4514 /wd4668 /wd4710 /wd4100 /wd5045)
else()
    add_compile_options(-Wall -Wextra -pedantic -Wno-unused-parameter)
endif()


if( WIN32 )
    set(SDL2_INCLUDE_DIRS "${PROJECT_SOURCE_DIR}/SDL2/include")

    if( CMAKE_SIZEOF_VOID_P EQUAL 8 )
        set(SDL2_LIBRARIES "${PROJECT_SOURCE_DIR}/SDL2/lib/x64/SDL2.lib")
    else()
        set(SDL2_LIBRARIES "${PROJECT_SOURCE_DIR}/SDL2/lib/x86/SDL2.lib")
    endif()
else()
    find_package(SDL2 REQUIRED)
endif()

include_directories(${SDL2_INCLUDE_DIRS})


add_executable(schach src/main.cpp
               src/board.cpp)
target_link_libraries(schach ${SDL2_LIBRARIES})


if( WIN32 )
    if( CMAKE_SIZEOF_VOID_P EQUAL 8 )
        add_custom_command(TARGET schach POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "${PROJECT_SOURCE_DIR}/SDL2/lib/x64/SDL2.dll"
            "$<TARGET_FILE_DIR:schach>/SDL2.dll")
    else()
        add_custom_command(TARGET schach POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "${PROJECT_SOURCE_DIR}/SDL2/lib/x86/SDL2.dll"
            "$<TARGET_FILE_DIR:schach>/SDL2.dll")
    endif()
endif()

add_custom_command(TARGET schach POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        "${PROJECT_SOURCE_DIR}/assets"
        "$<TARGET_FILE_DIR:schach>/assets")
