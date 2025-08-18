# Minimum CMake version required
cmake_minimum_required(VERSION 3.20)

# Project name and version
project(bearpswmng VERSION 1.0)
set(PROJECT_NAME bearpswmng)




# Set the C standard to C99 (or C11 if preferred)
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)




add_library(project_options INTERFACE)
add_library(library_options INTERFACE)

option(ENABLE_DEBUG "Enable Debug build" ON)

if (CMAKE_C_COMPILER_ID STREQUAL "Clang")
  # using Clang
elseif (CMAKE_C_COMPILER_ID STREQUAL "GNU")
  target_compile_options(project_options INTERFACE -Wall -Wextra -g)
  target_compile_options(library_options INTERFACE -Wall -Wextra -g)  
elseif (CMAKE_C_COMPILER_ID STREQUAL "Intel")
  # using Intel C++
elseif (CMAKE_C_COMPILER_ID STREQUAL "MSVC")
  target_compile_options(project_options INTERFACE /Zi /W3 /DEXE_TRACE)
  target_compile_options(library_options INTERFACE /Zi /W3 /DLIB_TRACE)
  target_link_options(project_options INTERFACE /subsystem:windows /entry:wWinMain)
  target_link_options(library_options INTERFACE /subsystem:windows /entry:DllMain)
set(CMAKE_C_FLAGS_DEBUG "/DDEBUG /Zi /W3")
set(CMAKE_C_FLAGS_RELEASE "/Ox")
  if(ENABLE_DEBUG)
    target_compile_options(library_options INTERFACE $<$<CONFIG:Debug>:${CMAKE_C_FLAGS_DEBUG}>)
  else()
    target_compile_options(library_options INTERFACE $<$<CONFIG:Release>:${CMAKE_C_FLAGS_RELEASE}>)
  endif()

endif()






#add_subdirectory("lib")
add_subdirectory("src")

