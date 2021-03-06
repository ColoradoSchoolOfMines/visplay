#ifndef VISPLAYCONTROLLER_H
#define VISPLAYCONTROLLER_H

#include <QObject>
#include <boost/thread/latch.hpp>

class VisplayController : public QObject
{
    Q_OBJECT
public:
    explicit VisplayController(QObject *parent = nullptr);
    bool ready_status;
    boost::latch *playback_latch;
    boost::latch *ready_latch;


Q_SIGNALS:
    void open_media(std::string file_path);

public Q_SLOTS:
};

#endif // VISPLAYCONTROLLER_H
