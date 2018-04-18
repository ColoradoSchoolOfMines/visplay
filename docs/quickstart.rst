Quickstart Guide
================

This guide is intended to get you started on developing Visplay as quickly as
possible.

Dependencies
------------

Make sure that you have the following installed and available on your system's
``PATH``.

- Python 3 (probably something like ``python3`` in your package manager)
- Pip 3 (probably something like ``python3-pip`` in your package manager)
- ``libmpv`` and ``mpv`` (depends on your package manager)
- ``youtube-dl`` (depends on your package manager)

Installation
------------

1. Clone the repository and install visplay::

        $ git clone https://github.com/ColoradoSchoolOfMines/visplay.git
        $ cd visplay
        $ pip3 install -e . --user

2. Run ``visplay``. It will prompt you to create the `Default Configuration`_.

.. note::

    If you have already installed, add ``--update`` to the ``pip install``
    command.

Configuration
-------------

By default, Visplay looks in ``$XDG_CONFIG_HOME``, and the ``$HOME/.config`` for
``visplay/visplay.yaml``. This is the main local configuration and points to
any number of ``assets.yaml`` files and ``playlists.yaml`` files (whether local
or remote).

You can use the ``-c`` or ``--config`` parameters to choose what configuration
file to use.

Default Configuration
^^^^^^^^^^^^^^^^^^^^^

To initialize the default configuration, run ``visplay`` without existing
configuration files. It will ask you if you want to generate the default ones.

If you want to override your existing configuration, delete your existing
configuration files and run ``visplay``.
