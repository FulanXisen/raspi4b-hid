# Raspberry Pi 4B as HID keyboard device

The following instructions will turn *Raspberry Pi 4B* into a HID keyboard to perform keystroke injection.

*Note*: 

- This was only verified on `4B` and `Windows 11`.

## How this works
1. Use RPi-4B's TYPC-C OTG port, let RPi act as a USB peripheral device.
    -  USB has HOST mode and DEVICE mode, RPi-4B's 2 USB2.0 and 2 USB3.0 port work in HOST mode by default. 
    -  Normally, RPi-4B doesn't have the ability to switch directly to USB device mode.
    -  OTG allows a device to be either a host or a device, and to switch between the two.
2. Create Keyboard device via HID

In order to send key pressed or released commands, we only need to write HID report to `/dev/hidg0`.

HID report is a 8-byte package. Please read more about HID report format.

- The first byte contains bit flags for some key like `Right-Ctrl`, `Left-Ctrl`, `Right-Shift`, `Left-Shift`, etc. If you want to press `Ctrl+Shift`, you need to add them together (`LEFT_CTRL+LEFT_SHIFT`) and put the value in to the first byte then write the 8-byte package to `/dev/hidg0`.

- The second byte is not used.

- To release a key write a 8-byte full of zeros package to `/dev/hidg0`

- The last 6 bytes are used to put your non-control keys. Theoretically, you can press 6 non-control keys with a 8-byte HID record. However, there is no guarantee all 6 keys will be pressed or pressed sequentially so just pack one key in one HID record, send it, and send a release record (full zeros record) to make sure a key is pressed and released properly.

- "control keys" here are refered to `Ctrl`, `Shift`, and `GUI` (Start key on Windows).

- I did compose most keycode in [`./HID/CODE/__init__.py`](./HID/CODE/__init__.py)

## Instructions

- Enable USB OTG (TYPE-C Cable on RPi4B) as peripheral：

Add `dtoverlay=dwc2` to `/boot/firmware/config.txt`

Add `dr_mode=peripheral` to `/boot/firmware/config.txt`

```bash
echo "dtoverlay=dwc2, dr_mode=peripheral" | sudo tee -a /boot/firmware/config.txt
```

- Load extra kernel modules

Add `dwc2` and `libcomposite` to `/etc/modules`

```bash
echo "dwc2" | sudo tee -a /etc/modules-load.d/modules.conf
echo "libcomposite" | sudo tee -a /etc/modules-load.d/modules.conf
```

- Restart to take effect

```
sudo reboot
```

- After reboot you should see things like

```
$ ls /sys/class/udc
fe980000.usb
```

- Config USB gadget to create a USB keyboard (a script is already in this repo)
  
```bash 
sudo bash create_keyboard.sh
```

- Now your PC should recognize a new USB Input Device

