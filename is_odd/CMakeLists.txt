cmake_minimum_required(VERSION 3.23)
project(IsOdd VERSION 0.1.0 LANGUAGES C)

set(CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/corrosion/cmake)

include(corrosion/cmake/Corrosion.cmake)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
add_library(IsOdd::is_odd ALIAS is_odd)

if(FALSE)
    target_sources(is_odd
            INTERFACE
            FILE_SET HEADERS
            BASE_DIRS include
            FILES
            include/is_odd/is_odd.h
    )
else()
    target_include_directories(is_odd INTERFACE
            $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
            $<INSTALL_INTERFACE:include>
    )
    target_sources(is_odd
            INTERFACE
            $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/is_odd/is_odd.h>
            $<INSTALL_INTERFACE:include/is_odd/is_odd.h>
    )
endif()

corrosion_install(TARGETS is_odd EXPORT ${PROJECT_NAME}Targets)
install(
        EXPORT ${PROJECT_NAME}Targets
        NAMESPACE ${PROJECT_NAME}::
        DESTINATION lib/cmake/${PROJECT_NAME}
)

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
        ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake
        ${CMAKE_CURRENT_BINARY_DIR}/corrosion/${PROJECT_NAME}TargetsCorrosion.cmake
        ${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake
        DESTINATION lib/cmake/${PROJECT_NAME}
)

