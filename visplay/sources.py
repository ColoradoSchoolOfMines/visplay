import yaml
import requests

# Every source needs to say whether the files it gets survive after an error,
# how to get an asset file, and how to get a playlist file.


def get_local_yaml(yaml_path):
    if yaml_path is None:
        return
    try:
        with open(yaml_path) as yaml_file:
            return yaml.load(yaml_file)

    except OSError:
        return {'error': 'An error parsing the yaml'}


class LocalSource:

    def __init__(self, assets_path=None, playlists_path=None):
        self.survive = True
        self.assets = get_local_yaml(assets_path)
        self.playlists = get_local_yaml(playlists_path)


class HTTPSource:

    def __init__(self, assets_path=None, playlists_path=None):
        try:
            if assets_path is not None:
                with requests.get(assets_path, verify=False) as remote_file:
                    self.assets = yaml.load(remote_file.content)

        except ConnectionError:
            return {'error': 'URL not available'}

        if playlists_path is not None:
            with requests.get(playlists_path, verify=False) as remote_file:
                self.playlists = yaml.load(remote_file.content)
