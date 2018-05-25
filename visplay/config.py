from os import environ, makedirs, path

import yaml


class Singleton(type):
    """Technique from here: https://stackoverflow.com/a/6798042/2319844"""
    _instances = {}

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super(Singleton,
                                        cls).__call__(*args, **kwargs)
        return cls._instances[cls]

    def __getattr__(cls, name):
        """Overrides dot operator access on the type."""
        if hasattr(cls._instances.get(cls), '__classgetattr__'):
            return cls._instances[cls].__classgetattr__(name)

        return super().__getattribute__(name)

    def __getitem__(cls, name):
        """Overrides blacket operator access on the type."""
        if hasattr(cls._instances.get(cls), '__getitem__'):
            return cls._instances[cls].__getitem__(name)

        return super().__getitem__(name)


class Config(metaclass=Singleton):
    """Global configuration object. This is a Singleton, so there is only ever
    one instance. Accessing configuration can be done using any of the
    following ways:

    >>> Config(foo='bar', baz='ohea')
    {'foo': 'bar', 'baz': 'ohea'}
    >>> Config.foo
    'bar'
    >>> Config['baz']
    'ohea'
    >>> Config.get('foo')
    'bar'
    >>> Config.get('ohea', 'cool')
    'cool'
    """
    # Default Configuration locations
    _config_folder = (environ.get('XDG_CONFIG_HOME') or
                      path.join(environ.get('HOME'), '.config'))
    _config_folder = path.join(_config_folder, 'visplay')
    default_config = path.join(_config_folder, 'config.yaml')

    def __repr__(self):
        return repr(self._dict)

    def __init__(self, *args, **kwargs):
        self._dict = dict(*args, **kwargs)

    def __classgetattr__(self, name):
        return self[name]

    def __getitem__(self, name):
        try:
            return self._dict[name]
        except KeyError as e:
            raise KeyError(f'No element {e} found in configuration.') from e

    @classmethod
    def get(cls, name, default=None):
        return cls()._dict.get(name, default)

    @classmethod
    def load_from_yaml(cls, config_file):
        cls()._dict.update(yaml.load(config_file))

    @classmethod
    def create_default_config(cls):
        """Automatically creates configuration files in the user's
        configuration directory.
        """
        # Create the config folder if it does not exist already.
        if not path.exists(cls._config_folder):
            makedirs(cls._config_folder)
            print(f'Folder {cls._config_folder} created')

        # Store the paths to the default locations.
        default_sources = path.join(cls._config_folder, 'root.sources.yaml')
        default_assets = path.join(cls._config_folder, 'assets.yaml')
        default_playlists = path.join(cls._config_folder, 'playlists.yaml')

        # Create the config file if it does not exist.
        if not path.exists(cls.default_config):
            with open(cls.default_config, 'w+') as f:
                f.write(f'sources: {default_sources}\n')
            print(f'File {cls.default_config} created')

        # Create the sources file if it does not exist.
        if not path.exists(default_sources):
            with open(default_sources, 'w+') as f:
                f.write('\n'.join([
                    'add:',
                    f'  - file:{default_assets}',
                    f'  - file:{default_playlists}',
                ]))
                f.write('\n')
            print(f'File {default_sources} created')

        # Create the assets file if it does not exist.
        if not path.exists(default_assets):
            with open(default_assets, 'w+') as f:
                f.write('intro: https://www.youtube.com/watch?v=9bZkp7q19f0\n')
            print(f'File {default_assets} created')

        # Create the playlists file if it does not exist.
        if not path.exists(default_playlists):
            with open(default_playlists, 'w+') as f:
                f.write('\n'.join([
                    'main:',
                    '  - intro',
                ]))
                f.write('\n')
            print(f'File {default_playlists} created')
