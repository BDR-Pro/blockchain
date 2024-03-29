#!/bin/bash

# Variables
VM_NAME="Windows 10"
VM_ISO_PATH="$HOME/Downloads/Win10_20H2_v2_English_x64.iso" # Change to your ISO path
VM_DIRECTORY="$HOME/VirtualBox VMs/$VM_NAME"

# Create VM directory if it doesn't exist
mkdir -p "$VM_DIRECTORY"

# Create and register the VM
VBoxManage createvm --name "$VM_NAME" --ostype Windows10_64 --register

# Modify VM settings
VBoxManage modifyvm "$VM_NAME" --cpus 2 --memory 4096 --vram 128 --graphicscontroller vmsvga --usbohci on --mouse usbtablet

# Create a virtual hard disk
VBoxManage createhd --filename "$VM_DIRECTORY/$VM_NAME.vdi" --size 50000 --variant Standard

# Add a SATA controller and attach the VDI
VBoxManage storagectl "$VM_NAME" --name "SATA Controller" --add sata --bootable on
VBoxManage storageattach "$VM_NAME" --storagectl "SATA Controller" --port 0 --device 0 --type hdd --medium "$VM_DIRECTORY/$VM_NAME.vdi"

# Add an IDE controller and attach the Windows ISO
VBoxManage storagectl "$VM_NAME" --name "IDE Controller" --add ide
VBoxManage storageattach "$VM_NAME" --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium "$VM_ISO_PATH"

# (Optional) Add an empty drive to simulate a place for Windows installation media or additional tools
VBoxManage storageattach "$VM_NAME" --storagectl "IDE Controller" --port 1 --device 0 --type dvddrive --medium emptydrive

# Start the VM in GUI mode
VBoxManage startvm "$VM_NAME" --type gui

# Note: After this point, you will need to manually proceed with the Windows installation process.
# This includes formatting the virtual hard disk during setup, entering a product key, and configuring initial settings.
