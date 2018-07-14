#ifndef GUI_H
#define GUI_H

#endif // GUI_H

#include <boost/python.hpp>

void init_gui(PyObject* callback);
void run_gui();
bool is_ready();
void setup_signals();

Q_DECLARE_METATYPE(std::string);