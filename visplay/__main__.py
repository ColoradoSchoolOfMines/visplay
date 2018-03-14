"""Main entrypoint for Visplay."""
from os import environ, path, makedirs
from queue import Queue
from argparse import ArgumentParser

from visplay import media, setupConfig
from visplay.sources import LocalSource, HTTPSource


def playable_generator(sources, messages):
    """Return a generator that will infinitely return new things to play."""
    running = True
    while running:
        playlist = []
        assets = [{}]
        prev_priority = "None"
        # Find the assets and playlists in each source
        for source in sources:
            # Get the asset first
            if source[0].assets:
                # Flatten sources with the same priority
                if source[1] == prev_priority:
                    asset[len(assets) - 1] = {**source[0].assets, **assets[len(assets) - 1]}
                else:
                    # Fill the first element so we don't end up with an empty one
                    if len(assets) == 0:
                        assets[0] = source[0].assets
                    else:
                        assets.append(source[0].assets)
            if source[0].playlists:
                if source[1] == prev_priority:
                    playlist += source[0].playlists
                else:
                    playlist = source[0].playlists
            prev_priority = source[1]
        # Consider changing the order in which sources are loaded
        assets.reverse()
        for playable in playlist:
            # if a message is sent telling this to reload, do it
            if not messages.empty():
                message = messages.get_nowait()
                if 'source' in message:
                    source = message['source']
            for asset in assets:
                if playable in asset:
                    yield asset[playable]
                    break


def main():
    """The main entrypoint for program when run standalone."""
    # Determine the config folder
    config_folder = environ.get('XDG_CONFIG_HOME') or path.join(environ.get('HOME'), '.config')
    if not path.exists(config_folder):
        makedirs(config_folder)

    parser = ArgumentParser()
    parser.add_argument('-c', '--config',
                        type=open,
                        default=path.join(config_folder, 'visplay.yaml'),
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

    media.findAndPlay(messages, playable_generator(sources, messages))


if __name__ == '__main__':
    main()
