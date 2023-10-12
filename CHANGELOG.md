# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Improved Error enums.
- Switched from manually written `dlopen` wrappers to bindgen generated `libloading`
    C bindings.


## 0.2.0 - 2023-10-10

### Changed

- Added an intermediate `FmuLibrary` type to distinguish between an `FmuInstance`
    and a loaded library to enable multiple parallel simulation instances to be run
    on the same library.

## 0.1.1 - 2023-10-09

### Fixed

- Mark `FmuInstance` as `Send`.

## 0.1.0 - 2023-10-09

Initial release 🎉