cmake_minimum_required(VERSION 3.23)
project(IsOdd VERSION 0.1.0 LANGUAGES C)

include(FetchContent)
FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.4
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

target_include_directories(is_odd
        INTERFACE
        include
)
# Not strictly necessary, but some IDEs like it
target_sources(is_odd INTERFACE include/is_odd/is_odd.h)
add_library(IsOdd::is_odd ALIAS is_odd)

install(DIRECTORY
        ${CMAKE_CURRENT_SOURCE_DIR}/include/is_odd
        DESTINATION include
)

if (BUILD_SHARED_LIBS)
    install(FILES $<TARGET_PROPERTY:is_odd-shared,IMPORTED_LOCATION> DESTINATION lib)

    set(FINAL_LIBRARY_NAME ${CMAKE_SHARED_LIBRARY_PREFIX}is_odd${CMAKE_SHARED_LIBRARY_SUFFIX})

    set(SHARED_LIBRARY_INIT
            "
add_library(${PROJECT_NAME}::is_odd SHARED IMPORTED)
set_target_properties(${PROJECT_NAME}::is_odd
        PROPERTIES
        IMPORTED_LOCATION \${PACKAGE_PREFIX_DIR}/lib/${FINAL_LIBRARY_NAME}
        IMPORTED_SONAME ${FINAL_LIBRARY_NAME}
)
"
    )
else ()
    install(FILES $<TARGET_PROPERTY:is_odd-static,IMPORTED_LOCATION> DESTINATION lib)

    set(FINAL_LIBRARY_NAME ${CMAKE_STATIC_LIBRARY_PREFIX}is_odd${CMAKE_STATIC_LIBRARY_SUFFIX})

    set(STATIC_LIBRARY_INIT
            "
add_library(${PROJECT_NAME}::is_odd STATIC IMPORTED)
set_target_properties(${PROJECT_NAME}::is_odd
        PROPERTIES
        IMPORTED_LOCATION \${PACKAGE_PREFIX_DIR}/lib/${FINAL_LIBRARY_NAME}
)
"
    )
endif ()

include(CMakePackageConfigHelpers)
write_basic_package_version_file(
        "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
        VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}"
        COMPATIBILITY AnyNewerVersion
)
configure_package_config_file(${CMAKE_CURRENT_SOURCE_DIR}/Config.cmake.in
        "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
        INSTALL_DESTINATION lib/cmake/${PROJECT_NAME}
        NO_SET_AND_CHECK_MACRO
        NO_CHECK_REQUIRED_COMPONENTS_MACRO
)
install(FILES
        "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
        ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake
        DESTINATION lib/cmake/${PROJECT_NAME}
)

