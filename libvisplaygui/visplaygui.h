#ifndef VISPLAYGUI_H
#define VISPLAYGUI_H

#include "visplay-gui_global.h"
#include "mpvwidget.h"

#include <QApplication>
#include <QObject>
#include <QMainWindow>
#include <QDialog>
#include <QVBoxLayout>
#include <QPushButton>
#include <QFileDialog>
#include <QtGui/QOpenGLContext>
#include <QPointer>

#include <string>
#include <boost/python.hpp>

#include <unistd.h>
#include <mpv/client.h>


class Q_DECL_EXPORT VisplayGui : public QObject
{
    Q_OBJECT

    public:
        VisplayGui(int argc, char *argv[]);
        ~VisplayGui();
        void display_gui();
        int argcc = 1;
        char *argvv[1] = {(char*)"test"};




        QPointer<QApplication>      app;
        // QPointer<QMainWindow>       win;
        QPointer<QWidget>           win;
        //QPointer<QWidget>           central_widget;
        //QPointer<QPushButton>       but;
        QPointer<MpvWidget>         mpv_widget;
        QPointer<QVBoxLayout>       vl;

    public Q_SLOTS:
        void open_media(std::string file_path);





};

#endif // VISPLAYGUI_H
