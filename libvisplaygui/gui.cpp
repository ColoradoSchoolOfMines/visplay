#include <QObject>

#include <iostream>
#include <boost/python.hpp>

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
    setup_signals();

    gui->display_gui();

    delete gui;

    // Restore GIL
    PyEval_RestoreThread(m_thread_state);
    m_thread_state = NULL;
}

void open_media(std::string file_path)
{
    Q_EMIT controller->open_media(file_path);
}

void setup_signals()
{
    controller = new VisplayController;

    QObject::connect(controller, &VisplayController::open_media, gui, &VisplayGui::open_media);

    // std::cout << "connected" << std::endl;

}



BOOST_PYTHON_MODULE(libvisplaygui)
{
    using namespace boost::python;
    def("init_gui", init_gui);
    def("open_media",  open_media);
}
