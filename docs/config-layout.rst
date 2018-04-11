Yaml Configuration Layout
=========================

- ``config.yaml``: Points source info, potentially in the future this can be
  used to configure other options.

  .. code:: yaml

      source: <path_to_sources.yaml>

- ``sources.yaml``: Imports sources or adds assets.

  .. code:: yaml

      import:
        usb: file:/some/path/to/sources.yaml
        acm: http://example.com/source.yaml
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
