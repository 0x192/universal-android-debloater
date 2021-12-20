# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The sections should follow the order `Packaging`, `Added`, `Changed`, `Fixed`
and `Removed`.

## [Unreleased]

### Added
- [[#52](https://github.com/0x192/universal-android-debloater/issues/52)] `uk.co.ee.myee` to the debloat lists  (thanks [@lawson58](https://github.com/lawson85)).
- [[#58](https://github.com/0x192/universal-android-debloater/issues/52)] `android` to the debloat lists with the tag `Unsafe`.
- [[#49](https://github.com/0x192/universal-android-debloater/issues/49)] Multi-device support: You are now able to select a device among the list of all ADB connected devices/emulators.
- [[#44](https://github.com/0x192/universal-android-debloater/issues/44)] Persistent settings: Settings (only `theme` for now) are saved to a config file. Its location follows [the standards of the different platforms](https://github.com/dirs-dev/dirs-rs#example).

### Changed
- Review of the package lists recommendations. The `Recommended` debloat list is now safer (less likely to remove something you'd want to keep).
- [[#65](https://github.com/0x192/universal-android-debloater/issues/65)] ADB commands now run in parallel and asynchronously! This means no more UI freeze when performing long/many actions! :rocket:
- UI now updates itself in real time when performing ADB actions (thanks to async & multithreading). Before, it waited for the end of all actions.

### Fixed
- Miscellaneous minor issues in some package descriptions.
- Several bad recommendations.
- [[#50](https://github.com/0x192/universal-android-debloater/issues/50)] Resync button flipping theme back to `Lupin`.

## [0.3] - 2021-10-10

### Added
- [[#16](https://github.com/0x192/universal-android-debloater/issues/16)] Multi-user support: You can now debloat/restore apps for any user of the phone (not only the primary user 0). 
   - `Multi user mode` setting (default to `on` for Android 5+) allowing to remove packages for all users ([a work profile is another user](https://developer.android.com/work/managed-profiles)) instead of only the selected user.
   - User switcher (picklist).
- [[#11](https://github.com/0x192/universal-android-debloater/issues/11)] New themes: light, dark and lupin. Lupin theme is now the new default theme. Themes can be changed from the settings.
- [[#40](https://github.com/0x192/universal-android-debloater/issues/40)] Description field scrollbar: you can now scroll long descriptions.

### Fixed
- [Regression] Unsafe packages can be deleted without enabling `expert mode`.
- The refresh button doesn't update settings when a (new) phone is connected.
- [Regression] Restore buttons are disabled when connecting an Android 8.0 phone.
- [[#17](https://github.com/0x192/universal-android-debloater/issues/17)] Refresh icon does not appear.

## [0.2.2] - 2021-09-30

### Fixed
- Crash when connecting a LG device (#33)

## [0.2.1] - 2021-09-28

### Added
- Software version in the navigation panel

### Packaging
- `wgpu` renderer is not the default renderer (you don't need to add `--features wgpu` if you want to build UAD with `wgpu`)

### Fixed
- [[#35](https://github.com/0x192/universal-android-debloater/issues/35)] Exported selection not found

## [0.2] - 2021-09-26

### Added
- [[#2](https://github.com/0x192/universal-android-debloater/issues/2)] UAD now comes with a logger. Debug information will be written to a `uad.log` file (Warning level log in *stdout*)
- [[#15](https://github.com/0x192/universal-android-debloater/issues/15)] Support for older phone (< Android 8.0):
- Disable mode in settings: clear and disable packages instead of uninstalling them (default for old phones because you can't restore uninstalled packages)
- [[#8](https://github.com/0x192/universal-android-debloater/issues/8)] Export your selection in the `uad_exported_selection.txt` file. Packages from this file (if found in the current directory) will be automatically selected upon the start of UAD (or after a refresh).

### Changed
- [[#25](https://github.com/0x192/universal-android-debloater/issues/25)] UAD will no longer crash at start if it doesn't find ADB but will display a useful error message
- [[#3](https://github.com/0x192/universal-android-debloater/issues/3)] Better handling of ADB errors
- Updated dependencies (compatibility with [Iced](https://github.com/iced-rs/iced) main branch latest commit)
- Cleanup and refactoring of the code
- Performance improvement
- Various UI/UX improvement
- The `Debloat/Restore selection` button has been split in 2 buttons: `removing` and `restoring`

### Packaging
- Added an alternative build that uses [OpenGL](https://fr.wikipedia.org/wiki/OpenGL) (instead of [Vulkan](https://fr.wikipedia.org/wiki/Vulkan_(API))) for compatibility with older computers. If you encouter some visual glitches with the default Vulkan build you should try the OpenGL build.

### Fixed
- Spelling mistake
- Failed build with MSVC toolchain
