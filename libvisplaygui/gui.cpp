#include <QObject>

#include <boost/python.hpp>
#include <boost/thread/latch.hpp>
#include <iostream>

#include <gui.h>
#include <visplaycontroller.h>
#include <visplaygui.h>

static VisplayGui* gui = NULL;
static VisplayController* controller = NULL;

/*
Python should pass a callback into init_gui that synchronizes
the main and the qt threads.
After calling init python should not call any libvisplaygui
function until the function provided in the callback argument
is called.
*/

void init_gui(PyObject* callback)
{
    // release GIL
    PyThreadState* m_thread_state = PyEval_SaveThread();

    qRegisterMetaType<std::string>();

    // create Visplay Qt Window
    int argc = 1;
    char* argv[] = { "test" };
    gui = new VisplayGui(argc, argv);

    controller = new VisplayController;

    boost::latch* playback_latch = new boost::latch(1);

    gui->playback_latch = playback_latch;
    controller->playback_latch = playback_latch;

    // Setup QT signals and slots
    setup_signals();

    /* 
        Pass in callback and thread state so the callback can
        be called when everthing is initalized.
    */
    m_thread_state = gui->display_gui(callback, m_thread_state);

    delete gui;

    // Restore GIL
    PyEval_RestoreThread(m_thread_state);
    m_thread_state = NULL;
}

void open_media(std::string file_path)
{
    Q_EMIT controller->open_media(file_path);

    controller->playback_latch->reset(1);
}

void wait_for_playback()
{

    controller->playback_latch->wait();
}

void setup_signals()
{

    QObject::connect(controller, &VisplayController::open_media, gui, &VisplayGui::open_media);
}

BOOST_PYTHON_MODULE(libvisplaygui)
{
    using namespace boost::python;
    def("init_gui", init_gui);
    def("open_media", open_media);
    def("wait_for_playback", wait_for_playback);
}
