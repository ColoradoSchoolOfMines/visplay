#!/usr/bin/env python3

# Python script to test libvisplaygui
# Compile libvisplaygui first
# Then run this script in the build directory with 'python -i qt_test.py'
# You can then make use of other library functions
# ex: libvisplaygui.open_media(file_path)

import libvisplaygui
from threading import Thread


new_thread = Thread(target=libvisplaygui.init_gui)
new_thread.daemon = True
new_thread.start()
