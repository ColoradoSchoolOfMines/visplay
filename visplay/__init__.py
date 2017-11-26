'visplay: from os import listdir'

from os import listdir
from os import path
import mpv


def my_log(loglevel, component, message):
    'my_log; acts as the log handler for the mpv object'
    print('[{}] {}: {}'.format(loglevel, component, message))


def main():
    'main: main entrypoint for program when run standalone'
#     player = mpv.MPV(log_handler=my_log, ytdl=True,
#                input_default_bindings=True, input_vo_keyboard=True)

    still_running = True

    video_path = path.expanduser('~/Videos/visplay')

    while still_running:
        print('Looking for videos in %s' % (video_path))
        video_map = find_videos(video_path)
        print('Found %d videos!' % (len(video_map)))

        if not video_map:
            print("Exiting...")
            exit(1)

        player = mpv.MPV(log_handler=my_log)
    #     player = mpv.MPV()
        player.fullscreen = True

        for video in video_map:
            player.play(video)
            player.wait_for_playback()


def find_videos(video_path):
    'findVideos: finds all video files in video path'
    return [video_path + '/' + file for file in listdir(video_path)]
    # return map(lambda a: video_path + "/" + a, listdir(video_path))


if __name__ == "__main__":
    main()
