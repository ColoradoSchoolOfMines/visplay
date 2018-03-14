import yaml
from os import environ, path, makedirs
from argparse import ArgumentParser


def load_config_yaml():
    '''load_config_yaml: loads config.yaml based on provided arguments
        or default location'''

    config_folder = environ.get('XDG_CONFIG_HOME') or path.join(
        environ.get('HOME'), '.config')
    config_folder = path.join(config_folder, 'visplay')
    if not path.exists(config_folder):
        makedirs(config_folder)

    parser = ArgumentParser()
    parser.add_argument('-c', '--config',
                        dest='config',
                        type=open,
                        default=path.join(config_folder, 'config.yaml'),
                        help='Specify a custom configuration file to load.')
    try:
        args = parser.parse_args()
    except Exception as e:
        print('Error parsing arguments:', e)
        exit(1)

    config = yaml.load(args.config)

    return config
