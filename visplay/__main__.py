"""Main entrypoint for Visplay."""
from os import environ, path, makedirs
from queue import Queue
from argparse import ArgumentParser

from visplay import media
from visplay.sources import LocalSource


def playable_generator(source, playlist, messages):
    """Return a generator that will infinitely return new things to play."""
    running = True
    while running:
        for playable in playlist['playlist']:
            # if a message is sent telling this to reload, do it
            if not messages.empty():
                message = messages.get_nowait()
                if 'source' in message:
                    source = message['source']
                elif 'playlist' in message:
                    playlist = message['playlist']
                    break
                else:
                    # The message was for someone else
                    messages.put(message)
            # get the asset pointed to by playlist
            yield source['assets'][playable]


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
    sourceList = {}
    sourceList['local'] = LocalSource()

    # TODO Don't use local for everything
    assets = sourceList['local'].get_assets(0)
    playlist = sourceList['local'].get_playlist(0)

    # Start mpv
    media.findAndPlay(messages, playable_generator(assets, playlist, messages))


if __name__ == '__main__':
    main()
