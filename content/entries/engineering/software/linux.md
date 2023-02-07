# linux

a collection of notes for sysadmin and general linux tasks that i find myself having to look up over and over again.

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

sometimes after that there are some random dpkg / apt errors:

```
sudo dpkg --configure -a
sudo apt --fix-broken install
```

## add desktop launcher

- example file in `/usr/share/applications/wally.desktop`

```
[Desktop Entry]
Type=Application
Name=Wally
Comment=USB Flasher for Ergodox EZ
Version=3.0.23
Exec=/usr/local/bin/wally
Icon=/opt/wally/appicon.png
Terminal=False
Categories=Firmware
```

## clean up

```
sudo apt-get clean
sudo apt-get autoremove --purge
```

# arch linux

These are some notes for myself when setting up arch.

## pacman

### setup

before you can use the package manager `pacman` you need to run:

```
pacman-key --init
```

This sets up your ability to set some keys and then use them to check the fingerprints on packages you're going to install.

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

# PopOS

Sometimes my system76 laptop's audio just stops working. This seems to fix it:

```
systemctl --user restart pulseaudio
rm -r ~/.config/pulse
pulseaudio -k
```

## inserting emojis

I can _never_ remember this:

- `Ctrl + Shift + E + Space`: then search via word tags

# disk space

```
df -h
du -h -d 1 . --exclude proc
```

journal logs

```
journalctl --rotate
journalctl --vacuum-size=100M
journalctl --vacuum-time=10d
```

docker

```
docker system prune -a
docker volume prune -a
```

snap

```
snap set system refresh.retain=2
```

# memory

sometimes you run out of memory on tiny machines. don't forget to use swap!

```shell
sudo fallocate -l 1G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile

sudo swapon /swapfile
sudo swapon --show

sudo cp /etc/fstab /etc/fstab.bak
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

cat /proc/sys/vm/swappiness
sudo sysctl vm.swappiness=10
```
