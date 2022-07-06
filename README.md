# Universal Android Debloater GUI
**DISCLAIMER**: Use at your own risk. I am not responsible for anything that could happen to your phone. 

<img src="/resources/screenshots/v0.5.0.png" width="850" alt="uad_screenshot">

**This software is still in an early stage of development. Check out the issues, and feel free to contribute!**

## Summary
This is a complete rewrite in Rust of the [UAD project](https://gitlab.com/W1nst0n/universal-android-debloater), which aims to improve privacy and battery performance by removing unnecessary and obscure system apps. This can also contribute to improve security by reducing [the attack surface](https://en.wikipedia.org/wiki/Attack_surface). 

Packages are as well documented as possible in order to provide a better understanding of what you can delete or not. The worst thing which could happen is removing an essential system package needed during boot causing then an unfortunate bootloop. After about 5 failed system boots, the phone will automatically reboot in recovery mode and you'll have to perform a FACTORY RESET. So make a backup first!

In any case, you can NOT brick your device with this software! That's the main thing, right?

## Features 
* [X] Uninstall/Disable and Restore/Enable system packages
* [X] Multi-user support (e.g apps in work profiles)
* [X] Export/Import your selection in `uad_exported_selection.txt`
* [X] Multi-device support: you can connect multiple phones at the same time
* [X] All your actions are logged so you never forget what you've done 

NB : System apps cannot really be uninstalled without root (see the [FAQ](https://gitlab.com/W1nst0n/universal-android-debloater-rs/-/wikis/FAQ))

## Universal debloat lists 
* [X] GFAM (Google/Facebook/Amazon/Microsoft)
* [X] AOSP
* [X] Manufacturers (OEM)
* [X] Mobile carriers
* [X] Qualcomm / Mediatek / Miscellaneous

## Manufacturers debloat lists
* [ ] Archos
* [X] Asus
* [ ] Blackberry
* [ ] Gionee
* [X] LG
* [X] Google
* [X] Fairphone
* [ ] HTC
* [X] Huawei
* [X] Motorola
* [X] Nokia
* [X] OnePlus
* [X] Oppo  
* [X] Samsung
* [X] Sony
* [X] Tecno
* [ ] TCL
* [ ] Wiko
* [X] Xiaomi
* [ ] ZTE

## Mobile carriers debloat lists
|   Country       | Carriers                          |
|-----------------|-----------------------------------|
| France          | Orange, SFR, Free, Bouygues       |
| USA             | T-Mobile, Verizon, Sprint, AT&T   |  
| Germany         | Telekom                           |
| UK              | EE                                |


## How to use it 
- **Read the [FAQ](https://github.com/0x192/universal-android-debloater/wiki/FAQ)!**
- **Do a proper backup of your data! You can never be too careful!**
- Enable *Developer Options* on your smartphone.
- Turn on *USB Debugging* from the developer panel.
- From the settings, disconnect from any OEM accounts (when you delete an OEM account package it could lock you on the lockscreen because the phone can't associate your identity anymore)
- Install ADB:
  <p>
  <details>
  <summary>LINUX</summary>

  - Install *Android platform tools* on your PC :

  Debian Base:
  ```bash
  $ sudo apt install android-sdk-platform-tools
  ```
  Arch-Linux Base:
  ```bash
  $ sudo pacman -S android-tools
  ```
  Red Hat Base:
  ```bash
  $ sudo yum install android-tools
  ```
  OpenSUSE Base:
  ```bash
  $ sudo zypper install android-tools
  ```
  </details>
  </p>

  <p>
  <details>
  <summary>MAC OS</summary>

  - Install [Homebrew](https://brew.sh/)
  - Install *Android platform tools*
    ```bash
    $ brew install android-platform-tools
    ```
  </details>
  </p>

  <p>
  <details>
  <summary>WINDOWS</summary>

  - Download [android platform tools](https://dl.google.com/android/repository/platform-tools-latest-windows.zip) and unzip it somewhere. [Add the folder to your PATH](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/).

  - [Install USB drivers for your device](https://developer.android.com/studio/run/oem-usb#Drivers)
  - Check your device is detected:
    ```batch
    > adb devices
    ```
  </details>
  </p>


- Download the latest release of UAD GUI for your Operating System [here](https://github.com/0x192/universal-android-debloater/releases). Take the `opengl` version only if the default version (with a Vulkan backend) doesn't launch.

**NOTE:** Chinese phones users may need to use the AOSP list for removing some stock apps because those Chinese manufacturers (especially Xiaomi and Huawei) have been using the name of AOSP packages for their own (modified & closed-source) apps.

**IMPORTANT NOTE:** You will have to run this software whenever your OEM pushes an update to your phone as some *uninstalled* system apps could be reinstalled.

## How to contribute

Hey-hey-hey! Don't go away so fast! This is a community project. That means I need you! I'm sure you want to make this project better anyway.

==> [How to contribute](https://github.com/0x192/universal-android-debloater/wiki)

## Special thanks

- [@mawilms](https://github.com/mawilms) for his LotRO plugin manager ([Lembas](https://github.com/mawilms/lembas)) which helped me a lot to understand how to use the [Iced](https://github.com/hecrj/iced) GUI library.
- [@casperstorm](https://github.com/casperstorm) for the UI/UX inspiration.
