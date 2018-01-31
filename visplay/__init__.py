'visplay: from os import listdir'

# import threading
from queue import Queue
# import rest
import media
import sources


def main():
    '''main: main entrypoint for program when run standalone'''
    # There are multiple threads so this allows them to communicate
    messages = Queue()

    # A list of sources following a basic interface. See sources.py
    sourceList = {}
    sourceList["local"] = sources.localSource()

    # TODO Don't use local for everything
    assets = sourceList["local"]["getAsset"](0)
    playlist = sourceList["local"]["getPlaylist"](0)

    # Start mpv
    media.findAndPlay(messages, playableGenerator(assets, playlist, messages))


# Returns a generator that will infinitely return new things to play
def playableGenerator(source, playlist, messages):
    running = True
    while running:
        for playable in playlist["playlist"]:
            # if a message is sent telling this to reload, do it
            if not messages.empty():
                message = messages.get_nowait()
                if "source" in message:
                    source = message["source"]
                elif "playlist" in message:
                    playlist = message["playlist"]
                    break
                else:
                    # The message was for someone else
                    messages.put(message)
            # get the asset pointed to by playlist
            yield source["assets"][playable]


if __name__ == "__main__":
    main()


