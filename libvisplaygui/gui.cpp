#include <QObject>

#include <iostream>
#include <boost/python.hpp>
#include <boost/thread/latch.hpp>

#include <visplaygui.h>
#include <visplaycontroller.h>
#include <gui.h>

static VisplayGui            *gui          = NULL;
static VisplayController     *controller   = NULL;


void init_gui()
{
    // release GIL
    PyThreadState     *m_thread_state = PyEval_SaveThread();

    qRegisterMetaType<std::string>();

    // init qt window
    int argc = 1;
    char *argv[] = {"test"};
    gui = new VisplayGui(argc, argv);
    controller = new VisplayController;
    boost::latch *ready_latch    = new boost::latch(1);
    boost::latch *playback_latch = new boost::latch(1);


    gui->playback_latch = playback_latch;
    gui->ready_latch = ready_latch;
    controller->playback_latch = playback_latch;
    controller->ready_latch = ready_latch;

    setup_signals();

    gui->display_gui();

    delete gui;

    // Restore GIL
    PyEval_RestoreThread(m_thread_state);
    m_thread_state = NULL;
}

void wait_until_ready()
{
   while(controller->ready_latch == NULL);
   controller->ready_latch->wait();
}

void open_media(std::string file_path)
{
    Q_EMIT controller->open_media(file_path);

    controller->playback_latch->reset(1);

}

void wait_for_playback() {

    controller->playback_latch->wait();
}

void setup_signals()
{

    QObject::connect(controller, &VisplayController::open_media, gui, &VisplayGui::open_media);

    // std::cout << "connected" << std::endl;

}



BOOST_PYTHON_MODULE(libvisplaygui)
{
    using namespace boost::python;
    def("init_gui",             init_gui);
    def("open_media",           open_media);
    def("wait_for_playback",    wait_for_playback);
    def("wait_until_ready",     wait_until_ready);
}
