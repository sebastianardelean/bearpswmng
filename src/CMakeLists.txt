
include(FetchContent)

set(INCLUDE_FILES "../include"
                  )





set(SOURCE_FILES main.c

)




include_directories(${INCLUDE_FILES})



add_executable(${PROJECT_NAME}  ${SOURCE_FILES})


target_link_libraries(
    ${PROJECT_NAME} PRIVATE
    project_options
)




