'visplay: from os import listdir'

from queue import Queue
import mpv
import configparser

messages = Queue()


def my_log(loglevel, component, message):
    'my_log; acts as the log handler for the mpv object'
    print('[{}] {}: {}'.format(loglevel, component, message))


player = mpv.MPV(log_handler=my_log, ytdl=True, input_vo_keyboard=True)


@player.on_key_press('q')
def my_q_binding():
    print("PressedQ")
    player.quit()
    messages.put("quit")


def main():
    '''main: main entrypoint for program when run standalone'''

    still_running = True
    config = configparser.ConfigParser()
    config.read('video.ini')

    while still_running:
        # Find all videos
        allVideos = {}
        for vid in config.sections():
            video = config[vid]
            mediaType = ''

            # Find the type of the video. Fail if not found
            print(vid)
            if 'type' in video:
                mediaType = video['type']
            else:
                print("Missing type in ini file")
                break

            # Find either the local or url of the video. Fail if not found
            if 'loc' in video:
                # TODO get better quality stuff by default
                allVideos[vid] = {}
                allVideos[vid]['loc'] = video['loc']
            else:
                print("Missing url or local in ini file")
                break

            allVideos[vid]['type'] = mediaType

        if not allVideos:
            print("Exiting...")
            exit(1)

        player.fullscreen = True

        # Play through all videos found in the ini file
        for video in allVideos:
            player.play(allVideos[video]['loc'])
            player.wait_for_playback()
            if not messages.empty() and messages.get_nowait() == "quit":
                exit(0)


if __name__ == "__main__":
    main()
