#include "visplaygui.h"
#include <iostream>
#include <mpv/client.h>
#include <boost/python.hpp>

static int salt = 0;

void init_gui() {

    std::cout << salt << std::endl;
}

BOOST_PYTHON_MODULE(libvisplaygui)
{
    using namespace boost::python;
    def("init_gui", init_gui);
}






Visplaygui::Visplaygui()
{


}
