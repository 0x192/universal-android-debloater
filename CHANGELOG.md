# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The sections should follow the order `Packaging`, `Added`, `Changed`, `Fixed`
and `Removed`.

## [Unreleased]

### Added
- UAD now comes with a logger. Debug information will be written to a `uad.log` file (Warning level log in *stdout*) (#2)
- Support for older phone (< Android 8.0) (#15):
- Disable mode in settings: clear and disable packages instead of uninstalling them (default for old phones because you can't restore uninstalled packages)

### Changed
- UAD will no longer crash at start if it doesn't find ADB but will display a useful message (#25)
- Better handling of ADB errors (#3)
- Updated dependencies (compatibility with [Iced](https://github.com/iced-rs/iced) main branch latest commit)
- Cleanup and refactoring of the code
- Performance improvement
- Refresh settings panel

### Fixed
- Spelling mistake
