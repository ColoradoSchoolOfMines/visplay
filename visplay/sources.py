import yaml
import requests
from visplay.setupConfig import get_sources_list
from visplay.setupConfig import sources_to_asset
from visplay.setupConfig import sources_to_play

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

    def __init__(self, as_asset=None, as_source=None):
        self.survive = True
        if as_source:
            source_path = as_source['source_path']
            constructors = as_source['construct']
            name = as_source['namespace']
            self.sources = get_sources_list(source_path, constructors)
            self.assets = sources_to_asset(name, self.sources)
            self.playlists = sources_to_play(name, self.sources)

        if as_asset:
            assets_path = as_asset['assets_path']
            playlists_path = as_asset['playlists_path']
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
