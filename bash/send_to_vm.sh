#!/bin/bash

# Initialize default values
VM_NAME=""
SNAPSHOT_NAME=""
USER=""
VM_IP=""
FILE_TO_SEND=""
EXECUTABLE_ON_VM=""
RESULT_FILE_ON_VM=""
RESULT_FILE_ON_HOST=""

# Function to display usage
usage() {
    echo "Usage: $0 --vm_name VM_NAME --snapshot_name SNAPSHOT_NAME --user USER --vm_ip VM_IP --file_to_send FILE_TO_SEND --executable_on_vm EXECUTABLE_ON_VM --result_file_on_vm RESULT_FILE_ON_VM --result_file_on_host RESULT_FILE_ON_HOST"
    exit 1
}

# Parse command-line options
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --vm_name) VM_NAME="$2"; shift ;;
        --snapshot_name) SNAPSHOT_NAME="$2"; shift ;;
        --user) USER="$2"; shift ;;
        --vm_ip) VM_IP="$2"; shift ;;
        --file_to_send) FILE_TO_SEND="$2"; shift ;;
        --executable_on_vm) EXECUTABLE_ON_VM="$2"; shift ;;
        --result_file_on_vm) RESULT_FILE_ON_VM="$2"; shift ;;
        --result_file_on_host) RESULT_FILE_ON_HOST="$2"; shift ;;
        *) usage ;;
    esac
    shift
done

# Check for required options
if [[ -z "$VM_NAME" || -z "$SNAPSHOT_NAME" || -z "$USER" || -z "$VM_IP" || -z "$FILE_TO_SEND" || -z "$EXECUTABLE_ON_VM" || -z "$RESULT_FILE_ON_VM" || -z "$RESULT_FILE_ON_HOST" ]]; then
    usage
fi

# Step 1: Send the file to the VM
scp $FILE_TO_SEND $USER@$VM_IP:$EXECUTABLE_ON_VM

# Step 2: Connect to the VM, execute the file, and save the result
ssh $USER@$VM_IP "bash $EXECUTABLE_ON_VM > $RESULT_FILE_ON_VM"

# Step 3: Retrieve the execution result
scp $USER@$VM_IP:$RESULT_FILE_ON_VM $RESULT_FILE_ON_HOST

# Step 4: Restore the VM snapshot
VBoxManage snapshot $VM_NAME restore $SNAPSHOT_NAME

# Start the VM again if needed
VBoxManage startvm $VM_NAME


## Make the script executable: chmod +x send_to_vm.sh.

## usage : ./send_to_vm.sh --vm_name "YourVMName" --snapshot_name "YourSnapshotName" 
##--user "vmUser" --vm_ip "vmIPAddress" --file_to_send "path/to/your/file" 
## --executable_on_vm "/path/on/vm/yourFile" --result_file_on_vm "/path/on/vm/result.txt" 
## --result_file_on_host "path/on/host/result.txt"
