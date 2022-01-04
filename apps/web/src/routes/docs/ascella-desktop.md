---
title: Ascella Desktop
description: Ascella Desktop app guide
---

# Ascella Desktop

- [Ascella Desktop](#ascella-desktop)
  - [Guide](#guide)
  - [Installing](#installing)
    - [Macos](#macos)
    - [Linux](#linux)

You can download the ascella desktop app from the [Ascella Releases](https://github.com/Tricked-dev/ascella/releases) tab.

![Gui Preview](/static/gui.png)

## Guide

After [installing](#Installing) Ascella desktop you get access to 4 commands

With no arguments provided it will open the ascella gui in where you can upload your config

```none
ascella 0.1.0
Tricked-dev
Ascella desktop app uploader

USAGE:
    ascella [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    area      Screenshot a area
    config    set the config file
    full      screenshot all screens
    help      Print this message or the help of the given subcommand(s)
    window    screenshot the current window
```

## Installing

### Macos

[Download](https://github.com/Tricked-dev/ascella/releases) & unzip the downloaded file

Place the `ascella` file in your `/bin` folder

```sh
chmod +x ./ascella && sudo mv ./ascella /bin/
```

Set the config file this can only be done via the commandline sadly

```sh
ascella config /path/to/ascella.sxcu
```

Optionally install [Flameshot](https://flameshot.org/) for a better screenshotting experience

And rebind your prnt+screen to `ascella area` or whatever you prefer see [this guide](https://appleinsider.com/articles/18/03/14/how-to-create-keyboard-shortcuts-to-launch-apps-in-macos-using-automator) for more info

### Linux

[Download](https://github.com/Tricked-dev/ascella/releases) & unzip the downloaded file

Place the `ascella` file in your `/bin` folder

```sh
chmod +x ./ascella && sudo mv ./ascella /bin/
```

Upload your config in the ascella menu and you should be set to start screenshotting!

install [scrot](https://github.com/resurrecting-open-source-projects/scrot), [flameshot](https://flameshot.org/), [Spectable](https://www.spectacleapp.com/), [gnome-screenshot](https://apps.gnome.org/app/org.gnome.Screenshot/) or [grim](https://github.com/emersion/grim) any of these screenshot tools should work with ascella if theres any issues please report them in the [Discord](https://discord.gg/mY8zTARu4g)

And rebind your prnt+screen to `ascella area` or whatever you prefer see [kde](https://www.addictivetips.com/ubuntu-linux-tips/customize-keyboard-shortcuts-on-kde-plasma-5/) or [gnome](https://help.gnome.org/users/gnome-help/stable/keyboard-shortcuts-set.html.en). although your probably better of looking through the settings yourself
