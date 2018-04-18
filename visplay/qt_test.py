#!/usr/bin/env python3

# Python script to test libvisplaygui
# Compile libvisplaygui first
# Then run this script in the build directory with 'python -i qt_test.py'
# You can then make use of other library functions
# ex: libvisplaygui.open_media(file_path)

import libvisplaygui
from threading import Thread
from time import sleep

new_thread = Thread(target=libvisplaygui.init_gui)
new_thread.setDaemon(True)
new_thread.start()

sleep(5)

libvisplaygui.wait_until_ready()

libvisplaygui.open_media("https://www.youtube.com/watch?v=NgmHghSOwHA")
libvisplaygui.wait_for_playback()

libvisplaygui.open_media("https://www.youtube.com/watch?v=NgmHghSOwHA")
libvisplaygui.wait_for_playback()
