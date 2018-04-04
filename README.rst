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

2. Create necessary configuration files as described in `Configuration`_.
3. Run `visplay`.

.. note::

    If you have already installed, add `--update` to the `pip install` command.

Configuration
-------------

By default, Visplay looks in ``$XDG_CONFIG_HOME``, and the ``$HOME/.config`` for
``visplay/visplay.yaml``. This is the main local configuration and points to
any number of ``assets.yaml`` files and ``playlists.yaml`` files (whether local
or remote)

See ``config.yaml.example``, ``sources.yaml.example``, ``assets.yaml.example``,
and ``playlists.yaml.example`` for example configurations.

To copy the configurations::

    mkdir ~/.config/visplay
    cp config.yaml.example ~/.config/visplay/config.yaml
    cp sources.yaml.example ~/.config/visplay/sources.yaml
    cp assets.yaml.example ~/.config/visplay/assets.yaml
    cp playlists.yaml.example ~/.config/visplay/playlists.yaml

Then replace ``"USER"`` in ``~/.config/config.yaml``, ``sources.yaml``,
``assets.yaml``, and ``playlists.yaml`` with your username. Also, edit
``assets.yaml`` to point to valid assets.

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

      source: Directory to sources.yaml

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
