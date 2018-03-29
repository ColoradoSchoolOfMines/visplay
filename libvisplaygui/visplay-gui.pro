#-------------------------------------------------
#
# Project created by QtCreator 2018-03-17T14:40:43
#
#-------------------------------------------------

QT       += widgets opengl svg webkit webkitwidgets dbus

LIBS     += -lboost_python3 -lpython3 -lmpv

CONFIG   += no_keywords

TARGET = visplaygui
TEMPLATE = lib

DEFINES += VISPLAYGUI_LIBRARY

# The following define makes your compiler emit warnings if you use
# any feature of Qt which has been marked as deprecated (the exact warnings
# depend on your compiler). Please consult the documentation of the
# deprecated API in order to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
        visplaygui.cpp \
    gui.cpp \
    mpvwidget.cpp \
    visplaycontroller.cpp

HEADERS += \
        visplaygui.h \
        visplay-gui_global.hbuild-visplay-gui-Desktop-Debug \
    gui.h \
    mpvwidget.h \
    visplaycontroller.h

unix {
    target.path = /usr/lib
    INSTALLS += target
}

INCLUDEPATH += /usr/include/python3.6m/
