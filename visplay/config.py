from argparse import ArgumentParser
from os import environ, makedirs, path
from pathlib import Path

import yaml

_config_folder = (environ.get('XDG_CONFIG_HOME') or
                  path.join(environ.get('HOME'), '.config'))
_config_folder = path.join(_config_folder, 'visplay')
_default_config = path.join(_config_folder, 'config.yaml')


def create_default_config():
    """Automatically creates configuration files in the user's home directory.
    """
    # Create the config folder if it does not exist already.
    if not path.exists(_config_folder):
        makedirs(_config_folder)
        print(f'Folder {_config_folder} created')

    # Store the paths to the default locations.
    default_sources = path.join(_config_folder, 'root.sources.yaml')
    default_assets = path.join(_config_folder, 'assets.yaml')
    default_playlists = path.join(_config_folder, 'playlists.yaml')

    # Create the config file if it does not exist.
    if not path.exists(_default_config):
        with open(_default_config, 'w+') as f:
            f.write(f'sources: {default_sources}\n')
        print(f'File {_default_config} created')

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


def load_config_yaml():
    """Loads config.yaml based on provided arguments or default location."""

    parser = ArgumentParser()
    parser.add_argument(
        '-c',
        '--config',
        dest='config',
        type=open,
        default=_default_config,
        help='Specify a custom configuration file to load.',
    )

    args = parser.parse_args()

    return yaml.load(args.config)
