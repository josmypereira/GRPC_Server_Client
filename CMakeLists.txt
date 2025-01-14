# ---------------------------------------------------------------- CMAKE MINIMUM VERSION
cmake_minimum_required(VERSION 3.9)

# ---------------------------------------------------------------- PROJECT NAME
project(protobuf-common)
set(GENERATE_PROJECT_NAME generate-${PROJECT_NAME})

# ----------------------------------------------------------------- FIND SOURCE
file(GLOB_RECURSE PROTOBUF_DEFINITION_FILES ${CMAKE_CURRENT_LIST_DIR}/*.proto)
file(GLOB_RECURSE SRC_FILES ${CMAKE_CURRENT_LIST_DIR}/*.cpp)

# -------------------------------------------------- ENSURE PROTOC IS INSTALLED
find_program(NATIVE_PROTOC protoc REQUIRED)
if(NOT NATIVE_PROTOC)
    message(WARNING "Please install protoc for compiling Protobuf definitions")
else()
# --------------------------------------------------------- GENERATE PROTO CODE
    foreach(PROTO_FILE ${PROTOBUF_DEFINITION_FILES})
        # Get the input and output filenames
        file(RELATIVE_PATH INPUT_FILE ${CMAKE_CURRENT_SOURCE_DIR} ${PROTO_FILE})
        get_filename_component(INPUT_FILE_DIRECTORY ${INPUT_FILE} DIRECTORY)
        get_filename_component(INPUT_FILE_NO_EXT ${PROTO_FILE} NAME_WE)
        set(OUTPUT_HEADER ${CMAKE_CURRENT_BINARY_DIR}/${INPUT_FILE_DIRECTORY}/${INPUT_FILE_NO_EXT}.pb.h)
        set(OUTPUT_SOURCE ${CMAKE_CURRENT_BINARY_DIR}/${INPUT_FILE_DIRECTORY}/${INPUT_FILE_NO_EXT}.pb.cc)

        # Use the protoc compiler to generate the protobuf source files
        set(PROTOBUF_ARGUMENTS -I="${CMAKE_CURRENT_SOURCE_DIR}"
                               --cpp_out="${CMAKE_CURRENT_BINARY_DIR}"
                               --python_out="${CMAKE_CURRENT_BINARY_DIR}"
                               "${INPUT_FILE}")
        add_custom_command(OUTPUT ${OUTPUT_HEADER} ${OUTPUT_SOURCE}
                           COMMAND ${NATIVE_PROTOC}
                           ARGS ${PROTOBUF_ARGUMENTS}
                           DEPENDS ${PROTO_FILE})

        # Keep track of the output files
        list(APPEND PROTOBUF_OUTPUT_FILES ${OUTPUT_HEADER} ${OUTPUT_SOURCE})
    endforeach()

    # A custom target that will trigger the generation of the protobuf files
    add_custom_target(${GENERATE_PROJECT_NAME} DEPENDS ${PROTOBUF_OUTPUT_FILES})

    # -------------------------------------------------------------- CREATE LIBRARY
    # An interface target that will allow other projects to include the generated
    # protobuf files
    add_library(${PROJECT_NAME} STATIC ${PROTOBUF_OUTPUT_FILES}
                                       ${SRC_FILES})
    add_dependencies(${PROJECT_NAME} ${GENERATE_PROJECT_NAME})

    # ----------------------------------------------------------- ADD INCLUDE PATHS
    target_include_directories(${PROJECT_NAME} SYSTEM PUBLIC ${CMAKE_CURRENT_BINARY_DIR})
    target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/device/include)

    # ----------------------------------------------------------- LINK DEPENDENCIES
    target_link_libraries(${PROJECT_NAME} PUBLIC libprotobuf)

endif()
