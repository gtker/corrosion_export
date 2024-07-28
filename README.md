# Corrosion Install Example

Simple Rust library built in cmake with [Corrosion](https://github.com/corrosion-rs/corrosion) and installed into a prefix.

Only tested on Linux, might need changes for Windows.

## Major points

You can install the actual library with:
```
if(BUILD_SHARED_LIBS)
    install(FILES $<TARGET_PROPERTY:is_odd-shared,IMPORTED_LOCATION> DESTINATION lib)
else()
    install(FILES $<TARGET_PROPERTY:is_odd-static,IMPORTED_LOCATION> DESTINATION lib)
endif()
```

But this will only install the static or shared library, not the public headers.
To do that we'll need to
```
install(DIRECTORY
        ${CMAKE_CURRENT_SOURCE_DIR}/include/is_odd
        DESTINATION include
)
```

### Installed Config

We can't use `install(EXPORT)` since for static libraries it creates an `INTERFACE IMPORTED`, while we need a `STATIC IMPORTED`.

## Usage

This library works both when installed and when `add_subdirectory`'d without any changes to depending code.

To test `add_subdirectory`:

* For static library run `cmake --preset add-subdir-static && cmake --build --preset add-subdir-static` and verify that the executable is built and works.
* For shared library run `cmake --preset add-subdir-shared && cmake --build --preset add-subdir-shared` and verify that the executable is built and works.

To test install:

In the `is_odd` directory, run:

* For static library `cmake -B build && cmake --build build && cmake --install build --install-prefix /tmp/install` and then `cd .. && cmake -DCMAKE_PREFIX_PATH="/tmp/install" --preset find-package-static && cmake --build --prefix find-package-static`
* For shared library `cmake -DBUILD_SHARED_LIBS=TRUE -B build && cmake --build build && cmake --install build --install-prefix /tmp/install` and then `cd .. && cmake -DCMAKE_PREFIX_PATH="/tmp/install" --preset find-package-static && cmake --build --prefix find-package-shared`


