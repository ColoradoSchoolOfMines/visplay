#include "visplaygui.h"
#include <iostream>
#include <thread>
#include <QDialog>
#include <QPushButton>
#include <QFileDialog>
#include <QtGui/QOpenGLContext>
#include <unistd.h>
#include <mpv/client.h>
#include <boost/python.hpp>

std::thread::id main_thread_id = std::this_thread::get_id();

static QApplication      *app;
static QMainWindow       *win;
static QPushButton       *but;

static PyThreadState     *m_thread_state;

static mpv_handle        *mpv;




void init_gui()
{
    // release GIL
    m_thread_state = PyEval_SaveThread();

    // init qt window
    int argc = 1;
    char *argv[] = {"test"};
    app = new QApplication(argc, argv);
    win = new QMainWindow();
    but = new QPushButton(win);
    // mpv_container = new QWidget(win);
    // mpv_container->setAttribute(Qt::WA_DontCreateNativeAncestors);
    // mpv_container->setAttribute(Qt::WA_NativeWindow);

    // init mpv
    // locale definintion required for mpv to function inside a qt env
    setlocale(LC_NUMERIC, "C");
    mpv = mpv_create();


    // int64_t wid = mpv_container->winId();
    // mpv_set_option(mpv, "wid", MPV_FORMAT_INT64, &wid);
    // QString filename = QFileDialog::getOpenFileName(win, "Open file");
    //QString filename("/home/motoko/Downloads/football.webm");
    // const QByteArray c_filename = filename.toUtf8();
    // const char *args[] = {"loadfile", c_filename.data(), NULL};
    //const char *args[] = {"loadfile", , NULL};
    // mpv_command_async(mpv, 0, args);

    win->show();

    app->exec();


    // Restore GIL
    PyEval_RestoreThread(m_thread_state);
    m_thread_state = NULL;
}

void run_gui()
{
}



BOOST_PYTHON_MODULE(libvisplaygui)
{
    using namespace boost::python;
    def("init_gui", init_gui);
    def("run_gui",  run_gui);
}






Visplaygui::Visplaygui()
{


}
