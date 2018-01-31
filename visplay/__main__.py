"""Main entrypoint for Visplay."""
from os import environ, path, makedirs
from queue import Queue
from argparse import ArgumentParser

from visplay import media, setupConfig
from visplay.sources import LocalSource, HTTPSource


def playable_generator(source, playlist, messages):
    """Return a generator that will infinitely return new things to play."""
    running = True
    while running:
        print(playlist)
        for playable in playlist:
            # if a message is sent telling this to reload, do it
            if not messages.empty():
                message = messages.get_nowait()
                if 'source' in message:
                    source = message['source']
            yield source[playable]


def main():
    """The main entrypoint for program when run standalone."""
    # Determine the config folder
    config_folder = environ.get('XDG_CONFIG_HOME') or path.join(environ.get('HOME'), '.config')
    if not path.exists(config_folder):
        makedirs(config_folder)

    parser = ArgumentParser()
    parser.add_argument('-c', '--config',
                        type=open,
                        default=path.join(config_folder, 'visplay.conf'),
                        help='Specify a custom configuration file to load.')
    try:
        args = parser.parse_args()
    except Exception as e:
        print('Error parsing arguments:', e)
        return

    # There are multiple threads so this allows them to communicate
    messages = Queue()

    # A list of sources following a basic interface. See sources.py
    constructors = {'local': LocalSource, 'http': HTTPSource}
    sources = setupConfig.get_source_list(args.config, constructors)

    assets = sources[0].assets
    playlist = sources[0].playlist

    # Start mpv
    media.findAndPlay(messages, playable_generator(assets, playlist, messages))


if __name__ == '__main__':
    main()
