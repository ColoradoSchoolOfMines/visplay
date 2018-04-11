#!/bin/bash
mkdir build
cd build

if [ "$1" == "distclean" ]; then
    qmake-qt5 ../visplay-gui.pro
    make distclean
    cd ..
    rm -r build
    exit
fi

qmake-qt5 ../visplay-gui.pro
make