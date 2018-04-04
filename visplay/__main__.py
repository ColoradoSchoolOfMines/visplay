"""Main entrypoint for Visplay."""
from queue import Queue

import prompt

from visplay import config, media, setupConfig
from visplay.setupConfig import sources_to_asset, sources_to_play
from visplay.sources import HTTPSource, LocalSource


def playable_generator(sources, messages):
    """Return a generator that will infinitely return new things to play."""
    running = True
    while running:
        playlist = ['main']
        assets = {}
        prev_priority = -1
        # Find the assets in each source and combine them
        for source in sources:
            if source.assets:
                assets = {**assets, **source.assets}

        # Build and play the stack
        while playlist:
            current = playlist.pop()
            if current in assets:
                if type(assets[current]) is list:
                    while assets[current]:
                        playlist.append(assets[current].pop())
                else:
                    yield assets[current]
            else:
                print("ERROR finding,", current)


def main():
    """The main entrypoint for program when run standalone."""

    # Calls the config function to get a dict of settings values
    while True:
        try:
            config_dict = config.load_config_yaml()
        except Exception as e:
            print(f'There was an error getting the configuration:\n\t{e}\n')
            print('Would you like to')
            print('\t(1) Exit')
            print('\t(2) Generate the default configuration (recommended)')
            response = prompt.regex(
                '^[12]$',
                prompt='Enter one of the keys in parentheses: ',
            )
            if response.string == '1':
                exit(0)
            else:
                config.create_default_config()
        else:
            # If everything worked, then we can proceed
            break

    # There are multiple threads so this allows them to communicate
    messages = Queue()

    # A list of sources following a basic interface. See sources.py
    constructors = {
        'file': LocalSource,
        'http': HTTPSource,
        'https': HTTPSource,
    }

    with open(config_dict['sources']) as source_file:
        sources = setupConfig.get_sources_list(source_file, constructors)

        media.find_and_play(messages, playable_generator(sources, messages))


if __name__ == '__main__':
    main()
