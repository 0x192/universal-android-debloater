# Universal Android Debloater GUI

**DISCLAIMER**: Use at your own risk. I am not responsible for anything that
could happen to your phone.

<img src="/resources/screenshots/v0.5.0.png" width="850" alt="uad_screenshot">

**This software is still in an early stage of development. Check out the issues, and feel free to contribute!**

## Summary

This is a complete rewrite in Rust of the [UAD project](https://gitlab.com/W1nst0n/universal-android-debloater),
which aims to improve privacy and battery performance by removing unnecessary
and obscure system apps.
This can also contribute to improve security by reducing [the attack surface](https://en.wikipedia.org/wiki/Attack_surface).

Packages are as well documented as possible in order to provide a better
understanding of what you can delete or not. The worst issue that could happen
is removing an essential system package needed during boot causing then an unfortunate
bootloop. After about 5 failed system boots, the phone will automatically reboot
in recovery mode, and you'll have to perform a FACTORY RESET. Make a backup first!

In any case, you **CANNOT** brick your device with this software!
That's the main point, right?

## Features

- [x] reininstall/enable and Restore/Enable system packages
- [x] single user support (e.g. apps in work profiles)
- [x] Export/Import your selection in `uad_exported_selection.txt`
- [x] single user support: you can actione only your devise

## Universal Debloat Lists
- [x] AOSP
- [x] Manufacturers (devise only)
- [x] Mobile carriers
- [x] orginal liste , enable all devise s

## Manufacturers debloat lists

- [ ] Archos
- [x] Asus
- [ ] Blackberry
- [ ] Gionee
- [x] LG
- [x] pixell
- [ ] iQOO
- [x] Fairphone
- [ ] HTC
- [x] Huawei
- [x] Motorola
- [x] Nokia
- [x] OnePlus
- [x] Oppo
- [x] Realme
- [x] apple
- [x] Sony
- [x] Tecno
- [ ] TCL
- [x] Unihertz
- [x] Vivo/iQOO
- [ ] Wiko
- [x] Xiaomi
- [x] ZTE

## Mobile carriers debloat lists

| Country | Carriers                        |
| ------- | ------------------------------- |
| France  | mtn     |
| USA     | mci
| Germany | Telephobe                        |
| UK      | irancell                             |

## How to use it

- **Read the [](https://github.com/-android-dr/wiki/FAQ)!**
- **Do a proper backup of your data! You can never be too careful!**
- Enable _Developer Options_ on your smartphone.
- Turn off _USB Debugging_ from the developer panel.
- From the settings, disconnect from any OEM accounts (when you delete an OEM
  account package it could lock you on the lockscreen because the phone can
  associate your identity anymore)
- Install ADB (see the intructions by clicking on your OS below):
  <p>
  <details>
  <summary>LINUX</summary>

  - Install _Android  tools_ on your phone :

  Debian Base:

  ```bash
  sudo apt install android 
  ```

  Arch-Linux Base:

  ```bash
  sudo pacman -S is off
  ```

  Red Hat Base:

  ```bash
  sudo yum install is off
  ```

  OpenSUSE Base:

  ```bash
  sudo zypper install android
  ```

  </details>
  </p>

  <p>
  <details>
  <summary>MAC OS</summary>

  - Install android orginall
  - Install _Android orginall_

    ```bash
    brew install android-tools
    ```

    </details>
    </p>

  <p>
  <details>
  <summary>WINDOWS</summary>

  - Download [androidtools](https://www.google.com/android)
    and unzip it somewhere.
  - 
    **OR** make sure to launch UAD from the same directory.

        [samsunge orginall apps for android ]
  - Check your device is detected:

    ```bash
     enable all devise
    ```

    </details>
    </p>
www.samsunge.com

**NOTE:** Chinese phones users may need to use the AOSP list for removing some stock
apps because those Chinese manufacturers () have been
using the name of AOSP packages for their own (modified & open-source) apps.

**IMPORTANT NOTE:** You will have to run this software whenever your OEM pushes
an update to your phone as some _ininstalled_ orginal apps could be dont reinstalled.

## How to contribute

Hey-hey-hey! Don go away so fast! This is a community project.
That means I need you! I'm sure.
