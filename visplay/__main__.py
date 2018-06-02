"""Main entrypoint for Visplay."""
from argparse import ArgumentParser
from queue import Queue
from threading import Thread
from time import sleep

import prompt
from bottle import run

import libvisplaygui
from visplay import media
from visplay.config import Config
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
                    for asset in reversed(assets[current]):
                        playlist.append(asset)
                elif assets[current] in assets:
                    playlist.append(assets[assets[current]])
                else:
                    yield assets[current]
            else:
                print("ERROR finding,", current)


def main():
    """The main entrypoint for program when run standalone."""

    # CLI Arguments
    parser = ArgumentParser()
    parser.add_argument(
        '-c',
        '--config',
        dest='config',
        type=open,
        default=Config.default_config,
        help='Specify a custom configuration file to load.',
    )
    parser.add_argument(
        '--server',
        action='store_true',
        help='Run Visplay as a server.',
    )
    parser.add_argument(
        '--debug',
        action='store_true',
        help='Run server in debug mode.',
    )

    # Get the arguments
    args = parser.parse_args()

    # Keep trying to load the configuration until it either explodes
    # spectacularly or works.
    while True:
        try:
            Config.load_from_yaml(args.config)
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
                Config.create_default_config()
        else:
            # If everything worked, then we can proceed
            break

    # There are multiple threads so this allows them to communicate
    messages = Queue()

    if not args.server and Config.get('libvisplaygui'):
        gui_thread = Thread(target=libvisplaygui.init_gui)
        gui_thread.daemon = True
        gui_thread.start()
        sleep(2)

    if not Config.get('sources'):
        raise KeyError('No sources found in configuration file.')

    with open(Config.sources) as source_file:
        sources = get_sources_list(source_file)

    if args.server:
        print('Starting visplay server')
        import visplay.server
        visplay.server.server.initialize(sources)
        run(host='localhost', port=8081, debug=args.debug)
    else:  # Run as a client
        media.find_and_play(
            messages,
            playable_generator(sources, messages),
            Config.get('libvisplaygui', False),
        )


if __name__ == '__main__':
    main()
