#!/bin/bash

# Variables
VM_NAME="Ubuntu 20.04"
VM_ISO_NAME="ubuntu-20.04.3-desktop-amd64.iso"
VM_DIRECTORY="$HOME/VirtualBox VMs/$VM_NAME"
EXT_PACK_NAME="Oracle_VM_VirtualBox_Extension_Pack-5.0.20-106931.vbox-extpack"
ISO_URL="https://releases.ubuntu.com/20.04.3/$VM_ISO_NAME"
EXT_PACK_URL="http://download.virtualbox.org/virtualbox/5.0.20/$EXT_PACK_NAME"

# Download Ubuntu VM .iso file and Extension Pack
wget $ISO_URL -P $VM_DIRECTORY
wget $EXT_PACK_URL -P $VM_DIRECTORY

# Install Extension Pack
VBoxManage extpack install "$VM_DIRECTORY/$EXT_PACK_NAME" --replace

# List installed Extension Packs to confirm installation
VBoxManage list extpacks

# Create and configure VM
VBoxManage createvm --name "$VM_NAME" --ostype Ubuntu_64 --register
VBoxManage modifyvm "$VM_NAME" --cpus 2 --memory 4096 --vram 128 --graphicscontroller vmsvga --usbohci on --mouse usbtablet
VBoxManage createhd --filename "$VM_DIRECTORY/$VM_NAME.vdi" --size 10240 --variant Standard
VBoxManage storagectl "$VM_NAME" --name "SATA Controller" --add sata --bootable on
VBoxManage storageattach "$VM_NAME" --storagectl "SATA Controller" --port 0 --device 0 --type hdd --medium "$VM_DIRECTORY/$VM_NAME.vdi"
VBoxManage storagectl "$VM_NAME" --name "IDE Controller" --add ide
VBoxManage storageattach "$VM_NAME" --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium "$VM_DIRECTORY/$VM_ISO_NAME"
VBoxManage setextradata "$VM_NAME" GUI/ScaleFactor 2
VBoxManage startvm "$VM_NAME" --type gui

# Note: The following commands require VirtualBox Guest Additions to be installed in the guest OS.
# You should run these commands manually in the VM's terminal after logging in for the first time.


VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S apt-get update
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S apt-get upgrade -y
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --environment "DISPLAY=:0" --wait-stdout --wait-stderr -- -S runuser -l ubuntu -c "gsettings set org.gnome.desktop.interface scaling-factor 2"
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S apt-get install openssh-server -y
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S systemctl enable ssh
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S systemctl start ssh



# Shutdown VM, take a snapshot, detach ISO, and reset GUI scale
VBoxManage controlvm "$VM_NAME" poweroff
VBoxManage snapshot "$VM_NAME" take "Ubuntu 20.04 snapshot"
VBoxManage storageattach "$VM_NAME" --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium emptydrive
VBoxManage setextradata "$VM_NAME" GUI/ScaleFactor 1
