#include "visplaygui.h"
#include "mpvwidget.h"

#include <iostream>
#include <thread>

#include <QString>
#include <QDialog>
#include <QVBoxLayout>
#include <QPushButton>
#include <QFileDialog>
#include <QtGui/QOpenGLContext>

#include <unistd.h>
#include <mpv/client.h>
#include <boost/python.hpp>


VisplayGui::VisplayGui(int argc, char *argv[])
{
    app         = new QApplication(argcc, argvv);
    //win         = new QMainWindow();
    win         = new QWidget;
    vl          = new QVBoxLayout();
    mpv_widget  = new MpvWidget(win);

    QObject::connect(mpv_widget, &MpvWidget::playback_idle, this, &VisplayGui::playback_idle);

    vl->addWidget(mpv_widget);
    vl->setContentsMargins(0, 0, 0, 0);
    win->setLayout(vl);

    //but         = new QPushButton(win);


}

VisplayGui::~VisplayGui() {

}

void VisplayGui::display_gui()
{
    win->show();
    ready_latch->count_down();
    app->exec();
}

void VisplayGui::open_media(std::string file_path)
{
    mpv_widget->command(QStringList() << "loadfile" << QString::fromStdString(file_path));
}

void VisplayGui::playback_idle() {
    playback_latch->count_down();
}

