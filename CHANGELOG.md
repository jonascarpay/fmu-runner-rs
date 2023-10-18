# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.0 - 2023-10-17

### Added

- Added `Fmu::variables()` method to easily get a reference to the signal map.

### Fixed

- Fixed `undefined symbol: fmi2EnterEventMode` error when loading FMU's that don't
    contain a ModelExchange model.

### Changed

- `FmuInstance` is now generic over a `Borrow<FmuLibrary>` to allow the user to
    managed `FmuLibrary` lifetime using Cell types.
- Refactored XML structs to deserialize directly to Enums.
- Made `FmuInstance::lib` public.

### Removed

- Removed `FmiModelDescription::map_signals()` in favor of `Fmu::variables()`.

## 0.3.0 - 2023-10-17

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