#!/bin/bash
mkdir build
cd build

if [ "$1" == "distclean" ]; then
    qmake ../visplay-gui.pro
    make distclean
    cd ..
    rm -r build
    exit
fi

qmake ../visplay-gui.pro
make