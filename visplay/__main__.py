"""Main entrypoint for Visplay."""
from os import environ, path, makedirs
from queue import Queue
from argparse import ArgumentParser

from visplay import media, setupConfig, config
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

    # Calls the config function to get a dict of settings values
    config_dict = config.load_config_yaml()

    # There are multiple threads so this allows them to communicate
    messages = Queue()

    # A list of sources following a basic interface. See sources.py
    constructors = {'local': LocalSource, 'http': HTTPSource}

    sources = setupConfig.get_sources_list(
        config_dict['sources'], constructors)

    media.findAndPlay(messages, playable_generator(sources, messages))


if __name__ == '__main__':
    main()
