#!/bin/bash

# Variables
VM_NAME="WinDev2401Eval"
ZIP_URL="https://aka.ms/windev_VM_vmware" # Replace this with your direct link
ZIP_FILE="$HOME/Downloads/$VM_NAME.zip"
EXTRACTED_OVA_PATH="$HOME/Downloads" # Adjust based on where you want to extract the ZIP contents

# Use curl with -L to follow redirects and download the ZIP file containing the OVA for the Windows development environment
curl -L "$ZIP_URL" -o "$ZIP_FILE"

# Assuming the download is a ZIP file; unzip it
unzip -o "$ZIP_FILE" -d "$EXTRACTED_OVA_PATH"

# Find the OVA file in the extracted directory
# This assumes the OVA file's name contains 'WinDev' and is the only OVA in the directory
OVA_FILE=$(find "$EXTRACTED_OVA_PATH" -type f -name "*WinDev*.ova")

# Import the OVA file into VirtualBox
VBoxManage import "$OVA_FILE" --vsys 0 --vmname "$VM_NAME"

# Start the imported VM
VBoxManage startvm "$VM_NAME" --type gui

# Note: Adjust extraction and import logic based on the actual content and structure of the ZIP file.
