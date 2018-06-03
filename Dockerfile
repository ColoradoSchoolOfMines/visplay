FROM ubuntu:bionic
MAINTAINER Visplay Dev Team

# Install dependencies
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y libmpv-dev python3 python3-dev libpython3-dev python3-pip qtbase5-dev qt5-default qt5-qmake libqt5svg5-dev libqt5webkit5-dev libboost-python-dev libboost-thread-dev


# Copy all of the current working directory to /visplay.
WORKDIR /visplay
ADD . /visplay

# Install Visplay
RUN pip3 install .
RUN cd libvisplaygui && ./build.sh