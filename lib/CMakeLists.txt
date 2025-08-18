
set(INCLUDE_FILES "../include"
)


include_directories(${INCLUDE_FILES})

set(SOURCE_FILES demo.c

)


add_library(demodll SHARED ${SOURCE_FILES})


target_link_libraries(demodll PRIVATE
	library_options
	kernel32
	user32
        ucrt    
        msvcrt

	)
# Define the export macro for Windows
target_compile_definitions(demodll PRIVATE MY_LIBRARY_EXPORTS)


