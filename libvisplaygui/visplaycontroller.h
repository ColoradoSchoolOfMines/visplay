#ifndef VISPLAYCONTROLLER_H
#define VISPLAYCONTROLLER_H

#include <QObject>

class VisplayController : public QObject
{
    Q_OBJECT
public:
    explicit VisplayController(QObject *parent = nullptr);

Q_SIGNALS:
    void open_media(std::string file_path);

public Q_SLOTS:
};

#endif // VISPLAYCONTROLLER_H
