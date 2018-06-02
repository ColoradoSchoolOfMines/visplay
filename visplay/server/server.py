from bottle import route

INITIALIZED = False
_sources = {}


def initialize(sources):
    global INITIALIZED, _sources
    INITIALIZED = True

    _sources = sources


@route('/')
def index():
    if not INITIALIZED:
        raise Exception('Visplay server is not initialized.')
    return _sources
