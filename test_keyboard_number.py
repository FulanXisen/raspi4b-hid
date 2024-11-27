import HID 
from HID import CODE

NUMBERS = [CODE.KEY_0,CODE.KEY_1,CODE.KEY_2,
CODE.KEY_3,CODE.KEY_4,CODE.KEY_5,
CODE.KEY_6,CODE.KEY_7,CODE.KEY_8,
CODE.KEY_9]
for i in range(len(NUMBERS)):
    HID.press(bytes([*[0]*2,NUMBERS[i],*[0]*5]))

