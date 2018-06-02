Project Organization
====================

Architecture Diagram
--------------------

Here is a diagram of the Visplay architecture. Each of the components of this
architecture are described below the diagram.

.. image:: diagrams/architecture.png
   :scale: 50%
   :align: center

**Visplay Server**

  .. TODO

  This doesn't exist yet. It will consist of the data protocol

**Visplay Client**

  .. TODO

**libvisplaygui**

  This is a library which interfaces directly with the hardware to display
  content to consumers. The goal of this library is to abstract the hardware
  details away from the client.

**HTTP(S) Server**

  .. TODO

Protocols
---------

There are two main protocols that Visplay is built around: the data protocol and
the display protocol.

.. TODO: describe protocols and link to protocol specs
