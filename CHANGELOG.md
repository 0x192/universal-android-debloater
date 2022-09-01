# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The sections should follow the order `Apps`, `Added`, `Changed`, `Fixed`, `Packaging`
and `Removed`.

## [0.5.1] - 2022-07-03

Since `0.5.0`, all changes related to apps are available to users without downloading a new version of UAD as the software directly download the json debloat list from Github. These changes can be tracked in commits with `[Pkg]` in their name. [See the commits](https://github.com/0x192/universal-android-debloater/commits/main)

### Added
- [[#209](https://github.com/0x192/universal-android-debloater/issues/209)] Persistent highlighting when you click on a package

### Changed
- `neededBy` and `dependencies` field can now list multiple packages (feature not visible in the UI yet)

### Fixed
- [[#286](https://github.com/0x192/universal-android-debloater/issues/286)] UAD stuck on "Downloading UAD lists. Please wait" screen

## Packaging
- [[#256](https://github.com/0x192/universal-android-debloater/issues/256)] Fixed typo in the release name of the noseflupdate variation
- Bump dependencies

## [0.5.0] - 2022-04-03

### Apps

- [[#115](https://github.com/0x192/universal-android-debloater/issues/115)] Added `com.tblenovo.lenovotips` to the recommended list.
- [[#120](https://github.com/0x192/universal-android-debloater/pull/120)] Move Google keyboard to `Advanced` list (Default keyboards should not be in the `Recommended` list)
- [[#169](https://github.com/0x192/universal-android-debloater/issues/154) Move `com.android.htmlviewer` to the `Expert` list. Removing it bootloop the device on MIUI 12.5.4+.

Huge thanks to [@KarlRamstedt](https://github.com/KarlRamstedt) for their help in this major debloat list update:
- [[#122](https://github.com/0x192/universal-android-debloater/pull/122)] Added a bunch of new packages
- [[#122](https://github.com/0x192/universal-android-debloater/pull/122)] A lot of description updates and fixes
- [[#122](https://github.com/0x192/universal-android-debloater/pull/122) | [#138](https://github.com/0x192/universal-android-debloater/pull/138)] Big revision of the recommendations according to more consistent criteria ([see the wiki](https://github.com/0x192/universal-android-debloater/wiki/FAQ#how-are-the-recommendations-chosen))

### Added
- [[#68](https://github.com/0x192/universal-android-debloater/issue/68)] **Unselect all button**: Let's you unselect all the packages you see on screen (i.e in the current filtered list).
- [[#119](https://github.com/0x192/universal-android-debloater/issue/119)] **Reboot button**: Let's you quickly reboot the currently selected device.
- [[#110](https://github.com/0x192/universal-android-debloater/pull/110)] **Remote `uad_lists.json` download**: The debloat list is now directly fetched from the main branch of this repo when you launch UAD. This means there is no longer the need to release a new version of UAD for updating the debloat lists! :rocket:
- [[#121](https://github.com/0x192/universal-android-debloater/pull/121)] :arrows_counterclockwise: **UAD self-update**: UAD will now check at launch if there is a new version of itself and enable you to perform the update directly from the app! :rocket:

### Changed
- [[#165](https://github.com/0x192/universal-android-debloater/issues/165)] UAD now tries every 500ms (for 1min) to initiate an ADB connection until a device is found during `FindingPhones`the loading state.
- All the init process was reworked and a status message is displayed at each stage (`DownloadingList`, `FindingPhones`,`LoadingPackages`,`UpdatingUad` `Ready`) so you know what is happening.
- Minor UI changes

### Packaging
- Add a `no-self-update` build for MacOS and Linux. Useful if UAD is distributed into repositories. The update process will then be managed by a package manager.
- MacOS builds are now also be released as a compressed tarball (like for Linux). You won't need to manually add the executable permission anymore. ([more info](https://github.com/actions/upload-artifact/issues/38))
- Bump dependencies


## [0.4.1] - 2022-01-31

### Fixed
- Selection counter never decreasing.

## [0.4] - 2022-01-30

### Apps
- [[#92](https://github.com/0x192/universal-android-debloater/pull/92)] Added 3 Fairphone packages + 7 Qualcomm packages (thanks [@VeH-c](https://github.com/VeH-c))
- [[#87](https://github.com/0x192/universal-android-debloater/pull/87)] Added 2 Unihertz packages (thanks [@rar0ch](https://github.com/rar0ch))
- [[#52](https://github.com/0x192/universal-android-debloater/issues/52)] Added `uk.co.ee.myee` to the debloat lists  (thanks [@lawson58](https://github.com/lawson85)).
- [[#58](https://github.com/0x192/universal-android-debloater/issues/52)] Added `android` to the debloat lists with the tag `Unsafe`.
- Added 2 new Xiaomi packages to the `Recommended` list.
- Multiple package description improvement (thanks [@jonas-ott](https://github.com/jonas-ott) and [@felurx](https://github.com/felurx) for the help)
- Review of the package lists recommendations. The `Recommended` debloat list is now safer (less likely to remove something you'd want to keep).

### Added
- [[#49](https://github.com/0x192/universal-android-debloater/issues/49)] Multi-device support: You are now able to select a device among the list of all ADB connected devices/emulators.
- [[#44](https://github.com/0x192/universal-android-debloater/issues/44)] Persistent settings: Settings (only `theme` for now) are saved to a config file. Its location follows [the standards of the different OS](https://github.com/dirs-dev/dirs-rs#example).
- Links to the Github page, wiki, github issues and logfiles in the `About` page.

### Changed
- [[#65](https://github.com/0x192/universal-android-debloater/issues/65)] ADB commands now run in parallel and asynchronously! This means no more UI freeze when performing long/many actions! :rocket:
- UI now updates itself in real time when performing ADB actions (thanks to async & multithreading). Before, it waited for the end of all actions.
- Logfiles are now located in a more conventional place: [cache_dir](https://docs.rs/dirs/latest/dirs/).
- Previous logs are no longer overwritten. The logger now only appends to the current logfile of the day (UAD_%Y%m%d.log).
- Each new day the logger will create a new file on UAD launch.
- [[#78](https://github.com/0x192/universal-android-debloater/issues/78)] Disable mode is now only available on Android 6+ because the disable ADB commands do not work without root on older devices. The setting will be greyed-out for those devices.
- Minor light theme update


### Fixed
- [[#50](https://github.com/0x192/universal-android-debloater/issues/50)] Resync button flipping theme back to `Lupin`.
- [Regression ([048e7f](https://github.com/0x192/universal-android-debloater/commit/048e7fc8fd6d44b0e8ba933c289249366254a9cc))] Weird disabled/greyed action button with older devices (< Android 8.0). Package could be selected but no action was performed.
- [[#78](https://github.com/0x192/universal-android-debloater/issues/78)] Packages not being actually uninstalled on older devices (< Android 6.0). Without root we can only use `pm block`/`pm unblock` for Android KitKit (4.4) and `pm hide`/`pm unhide` on Android Lollipop (5.x).

### Packaging
- For Arch-based users, UAD is now available in the AUR: `universal-android-debloater-bin` (binary) and `universal-android-debloater` (from source)
- Bump dependencies


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
