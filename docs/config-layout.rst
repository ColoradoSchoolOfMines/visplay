.. Highlight as YAML by default
.. highlight:: yaml

Yaml Configuration Layout
=========================

- ``config.yaml``: Points source info, gui backend configuration,
  potentially in the future this can be used to configure other options::

      source: <path_to_sources.yaml>
      libvisplaygui: True/False

- ``sources.yaml``: Imports sources or adds assets::

      import:
        usb: file:/some/path/to/sources.yaml
        acm: http://example.com/source.yaml
      add:
        - file:/home/USER/.config/visplay/assets.yaml
        - file:/home/USER/.config/visplay/playlists.yaml
        - http://example.net/path/playlists.yaml

  Supported source types:

  - ``file`` and ``path``: imports from a path (supports files and directories)
  - ``http`` and ``https``: imports from a URL

- ``assets.yaml``: Provides video assets::

      TheAssetName: <local_path_or_url>
      CoolestVidEver: https://www.youtube.com/watch?v=dQw4w9WgXcQ

- ``playlists.yaml``: Provides assets that play other assets::

      main:
        - CoolestVidEver
        - TheAssetName
