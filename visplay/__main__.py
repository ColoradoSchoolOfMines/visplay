"""Main entrypoint for Visplay."""
from queue import Queue

from visplay import media, setupConfig, config
from visplay.sources import LocalSource, HTTPSource
from visplay.setupConfig import sources_to_asset
from visplay.setupConfig import sources_to_play


def playable_generator(sources, messages):
    """Return a generator that will infinitely return new things to play."""
    running = True
    while running:
        playlist = []
        assets = {}
        prev_priority = -1
        # Find the assets and playlists in each source
        for source in sources:
            if source.playlists and source.priority > prev_priority:
                playlist = source.playlists
            if source.assets:
                assets = {**assets, **source.assets}
        for playable in playlist:
            # if a message is sent telling this to reload, do it
            if not messages.empty():
                message = messages.get_nowait()
                if 'source' in message:
                    source = message['source']
            yield assets[playable]


def main():
    """The main entrypoint for program when run standalone."""

    # Calls the config function to get a dict of settings values
    config_dict = config.load_config_yaml()

    # There are multiple threads so this allows them to communicate
    messages = Queue()

    # A list of sources following a basic interface. See sources.py
    constructors = {'local': LocalSource, 'http': HTTPSource}

    with open(config_dict['sources']) as source_file:
        sources = setupConfig.get_sources_list(
            source_file, constructors)

        media.find_and_play(messages, playable_generator(sources, messages))


if __name__ == '__main__':
    main()
