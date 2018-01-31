from queue import Queue
from flask import Flask
from flask import request


messenger = 1

SETTINGS = {
    'DEBUG': True,
    'DOMAIN': {'test': {}}
}


# app = Flask(settings=SETTINGS)
app = Flask(__name__)


@app.route('/hello')
def pause():
    print("hi")


@app.route('/shutdown')
def shutdown():
    # messenger.put("quit")
    func = request.environ.get('werkzeug.server.shutdown')
    if func is None:
        raise RuntimeError('Not running with the Werkzeug Server')
    func()
    return "Shutdown"


def start(messages):
    messenger = messages
    # messenger.put("quit")
    try:
        app.run(host="0.0.0.0")
    except:
        messenger.put("quit")
