"""Main entrypoint for Visplay."""
from queue import Queue
from threading import Thread
from time import sleep

import prompt

import libvisplaygui

from visplay import config, media
from visplay.setup_sources import get_sources_list


def playable_generator(sources, messages):
    """Return a generator that will infinitely return new things to play."""
    running = True
    while running:
        playlist = ['main']
        assets = {}

        # Find the assets in each source and combine them
        for source in sources:
            if source.assets:
                assets.update(source.assets)

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

    # Keep trying to load the configuration until it either explodes
    # spectacularly or works.
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

    if config_dict.get('libvisplaygui', False):
        gui_thread = Thread(target=libvisplaygui.init_gui)
        gui_thread.daemon = True
        gui_thread.start()
        sleep(2)

    with open(config_dict['sources']) as source_file:
        sources = get_sources_list(source_file)
        media.find_and_play(
            messages,
            playable_generator(sources, messages),
            config_dict.get('libvisplaygui', False),
        )


if __name__ == '__main__':
    main()
