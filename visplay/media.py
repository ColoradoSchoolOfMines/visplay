import mpv
import libvisplaygui

# TODO allow passing of message queue in better way
messengerDic = {}


def my_log(loglevel, component, message):
    """Acts as the log handler for the mpv object"""
    print('[{}] {}: {}'.format(loglevel, component, message))


player = mpv.MPV(log_handler=my_log, input_default_bindings=True, ytdl=True,
                 input_vo_keyboard=True)


# def callback():
#     print("hello")


# player.register_message_handler("add_periodic_timer", 5, callback)


# Quit on q
@player.on_key_press('q')
def my_q_binding():
    player.quit()
    messengerDic['queue'].put('quit')


# Pause on p
@player.on_key_press('p')
def my_p_binding():
    player.pause = not player.pause


def find_and_play(messages, generator, libvisplaygui_enabled=False):
    '''find_and_play: gets videos from the generator and plays them'''

    # Pass the keypress functions the queue
    messengerDic['queue'] = messages

    player.fullscreen = True

    # Loop through videos from the generator
    for video in generator:

        if libvisplaygui_enabled:
            libvisplaygui.open_media(video)
            libvisplaygui.wait_for_playback()
        else:
            player.play(video)
            player.wait_for_playback()

        if not messages.empty() and messages.get_nowait() == 'quit':
            exit(0)
