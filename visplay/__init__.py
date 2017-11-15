import mpv
from os import listdir

def my_log(loglevel, component, message):
      print('[{}] {}: {}'.format(loglevel, component, message))

def main():
      #player = mpv.MPV(log_handler=my_log, ytdl=True, input_default_bindings=True, input_vo_keyboard=True)
      player = mpv.MPV(log_handler=my_log)
      #player = mpv.MPV()
      player.fullscreen = True
      stillRunning = True

      while stillRunning:
            for v in findVideos('/home/jack/Videos/visplay'):
                  player.play(v)
                  player.wait_for_playback()

def findVideos(path):
      return map(lambda a: path + "/" + a, listdir(path))

if __name__ == "__main__":
      main()
