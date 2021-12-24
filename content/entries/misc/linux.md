# ubuntu

## dist upgrade

```
sudo apt update
sudo apt upgrade
sudo apt dist-upgrade
sudo apt autoremove
sudo do-release-upgrade
sudo reboot now
```

# arch linux

These are some notes for myself when setting up arch.

## pacman

### setup

before you can use the package manager `pacman` you need to run:

```
pacman-key --init
```

This sets up your abilit to set some keys and then use them to check the fingerprints on packages you're going to install.

If you're on a raspberry pi, you'll need to add the keys for `archlinuxarm` in particular:

```
pacman-key --populate archlinuxarm
```

Thereafter you can go about installing whatever you need.

If you need to remove all the keys you have added to start over, you can:

```
rm -rf /etc/pacman.d/gnupg
```

before running the init again.

## base packages

In order to get make, and a bunch of basic tooling:

```
pacman -S base-devel
```

## AUR client

I always need to download some AUR packages. Usually we still need to download a client:

```
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si
```
