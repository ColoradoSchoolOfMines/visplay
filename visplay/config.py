from argparse import ArgumentParser
from os import environ, makedirs, path

import yaml


def load_config_yaml():
    """Loads config.yaml based on provided arguments or default location."""

    config_folder = (environ.get('XDG_CONFIG_HOME') or
                     path.join(environ.get('HOME'), '.config'))
    config_folder = path.join(config_folder, 'visplay')
    if not path.exists(config_folder):
        makedirs(config_folder)

    default_config = path.join(config_folder, 'config.yaml')

    parser = ArgumentParser()
    parser.add_argument(
        '-c',
        '--config',
        dest='config',
        type=open,
        default=default_config,
        help='Specify a custom configuration file to load.',
    )

    try:
        args = parser.parse_args()
    except FileNotFoundError:
        print(f'ERROR: Configuration file not found at {default_config}')
        exit(1)
    except Exception as e:
        print('Error parsing arguments:', e)
        exit(1)

    config = yaml.load(args.config)

    return config
