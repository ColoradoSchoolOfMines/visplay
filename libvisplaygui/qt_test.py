#!/usr/bin/env python3

import libvisplaygui
from threading import Thread


new_thread = Thread(target=libvisplaygui.init_gui)
new_thread.setDaemon(True)
new_thread.start()
