#!/bin/bash

# Variables
VM_NAME="Ubuntu 20.04 Server"
OVA_URL="https://sourceforge.net/projects/osimages/files/Linux/Ubuntu/Ubuntu%2020.04%20LTS%20Server%20x64.ova/download"
OVA_FILE="$HOME/Downloads/Ubuntu_20.04_LTS_Server_x64.ova" # Adjust as necessary
EXT_PACK_NAME="Oracle_VM_VirtualBox_Extension_Pack-5.0.20-106931.vbox-extpack"
EXT_PACK_URL="http://download.virtualbox.org/virtualbox/5.0.20/$EXT_PACK_NAME"

# Download the OVA file
wget -O "$OVA_FILE" "$OVA_URL"

# Download Extension Pack
wget $EXT_PACK_URL -P "$HOME/Downloads"

# Install Extension Pack
VBoxManage extpack install "$HOME/Downloads/$EXT_PACK_NAME" --replace

# List installed Extension Packs to confirm installation
VBoxManage list extpacks

# Import OVA
VBoxManage import "$OVA_FILE" --vsys 0 --vmname "$VM_NAME"

# Modify VM settings (optional, adjust as needed)
VBoxManage modifyvm "$VM_NAME" --cpus 4 --memory 4096 --vram 512

# Start VM
VBoxManage startvm "$VM_NAME" --type gui

VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S apt-get update
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S apt-get upgrade -y
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --environment "DISPLAY=:0" --wait-stdout --wait-stderr -- -S runuser -l ubuntu -c "gsettings set org.gnome.desktop.interface scaling-factor 2"
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S apt-get install openssh-server -y
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S systemctl enable ssh
VBoxManage guestcontrol "$VM_NAME" run --exe "/usr/bin/sudo" --username ubuntu --password ubuntu --wait-stdout --wait-stderr -- -S systemctl start ssh


# wait and then shutdown

sleep 60

VBoxManage controlvm "$VM_NAME" poweroff
