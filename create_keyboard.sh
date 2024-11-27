#!/bin/bash
# create_keyboard.sh

# Create new device configuration
cd /sys/kernel/config/usb_gadget/
mkdir -p g1
cd g1

# Set basic information
echo 0x1d6b > idVendor # Linux Foundation
echo 0x0100 > idProduct # Multifunction Composite Gadget
echo 0x0200 > bcdDevice # v2.0.0
echo 0x0200 > bcdUSB # USB2

# Set device string desc
mkdir -p strings/0x409
echo "fanyx0987654321" > strings/0x409/serialnumber
echo "RPi Inc" > strings/0x409/manufacturer
echo "RPi USB Device" > strings/0x409/product

# Create configurations
mkdir -p configs/c.1

# Create HID device
mkdir -p functions/hid.usb0

# Set device
echo 1 > functions/hid.usb0/protocol
echo 1 > functions/hid.usb0/subclass
echo 8 > functions/hid.usb0/report_length
echo -ne \\x05\\x01\\x09\\x06\\xa1\\x01\\x05\\x07\\x19\\xe0\\x29\\xe7\\x15\\x00\\x25\\x01\\x75\\x01\\x95\\x08\\x81\\x02\\x95\\x01\\x75\\x08\\x81\\x03\\x95\\x05\\x75\\x01\\x05\\x08\\x19\\x01\\x29\\x05\\x91\\x02\\x95\\x01\\x75\\x03\\x91\\x03\\x95\\x06\\x75\\x08\\x15\\x00\\x25\\x65\\x05\\x07\\x19\\x00\\x29\\x65\\x81\\x00\\xc0 > functions/hid.usb0/report_desc

# Add HID device to configurations
ln -s functions/hid.usb0 configs/c.1/

# Binding to USB Device Controller
ls /sys/class/udc > UDC