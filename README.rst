Visplay
=======

This project's goal is to make the TV's in the foyer of the buildings of Mines
display media of ACM's projects. The project uses MPV and python.

Features
--------

Stream videos and open media files using an ``mpv`` implementation.

Installation
------------

1. Clone the repository and install visplay::

        $ git clone https://github.com/ColoradoSchoolOfMines/visplay.git
        $ cd visplay
        $ pip3 install -e . --user

2. Run ``visplay``. It will prompt you to create the `Default Configuration`_.

.. note::

    If you have already installed, add `--update` to the `pip install` command.

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

Dependencies
------------

- Python 3 (probably something like `python3` in your package manager)
- Pip 3 (probably something like `python3-pip` in your package manager)
- ``libmpv`` and ``mpv`` (depends on your package manager)
- ``youtube-dl``

Documentation
-------------
Yaml Configuration Layout
^^^^^^^^^^^^^^^^^^^^^^^^^

- ``config.yaml``: Points source info, potentially in the future this can be
  used to configure other options.

  .. code:: yaml

      source: <path_to_sources.yaml>

- ``sources.yaml``: Imports sources or adds assets.

  .. code:: yaml

      import:
        - usb: file:/some/path/to/sources.yaml
        - acm: http://example.com/source.yaml
      add:
        - file:/home/USER/.config/visplay/assets.yaml
        - file:/home/USER/.config/visplay/playlists.yaml
        - http://example.net/path/playlists.yaml

- ``assets.yaml``: Provides video assets.

  .. code:: yaml

      TheAssetName: local path or url
      CoolestVidEver: https://www.youtube.com/watch?v=dQw4w9WgXcQ

- ``playlists.yaml``: Provides assets that play other assets.

  .. code:: yaml

      main:
        - CoolestVidEver
        - TheAssetName
