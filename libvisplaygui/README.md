## libvisplaygui
This library abstracts Visplay's windowing logic and the playback implementation to a QT based C++ library with Python bindings via boost-python3

## Features
Wraps a libmpv video output in a Qt widget (thanks to the qt-opengl example
code in the libmpv repo) and builds a Qt application around it. Future plans
include allowing metadata display and other advanced windowing operations.

## Compilation
Run qtcreator, when the big warning dialog comes up about lib visplay **click NO**. Then under build click rebuild all.

To test:

    cd build-visplay-gui-* #if in root visplay folder
    python3 -i qt_test.py
    
If you get a blank black screen then you are good

## Documentation
The mpv and Qt instances in this library are designed to be run in a seperate thread (see examples) and then called into using the provided functions, which handle cross thread communication using Qt's signal and slots mechanism.

Function:
    init_gui(): creates the Qt window and mpv instance and displays them
    open_media(string):  tells the MPV instance to play the specified
                         media

## Examples

The following code will create a Qt window with an mpv widget and then play a
youtube video in it (requires youtube-dl)

    import libvisplaygui
    from threading import Thread

    new_thread = Thread(target=libvisplaygui.init_gui)
    new_thread.setDaemon(True)
    new_thread.start()

    libvisplaygui.open_media("https://www.youtube.com/watch?v=1hJKhiew2O0")


## Dependencies

libmpv / mpv (depends on distro, some bundle them)

youtube-dl (support for YouTube and other streaming sites)

qt5, qmake-qt5

boost-python3


