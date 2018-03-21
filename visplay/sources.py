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
            with open(source_path) as source_file:
                # Recursively discover all sources
                self.sources = get_sources_list(source_file, constructors)
                # Namespace the assets
                self.assets = sources_to_asset(name, self.sources)

        if as_asset:
            assets_paths = as_asset['assets_paths']
            self.assets = {}
            for asset in assets_paths:
                self.assets = {**self.assets, **get_local_yaml(asset)}


class HTTPSource:

    def __init__(self, as_asset=None, as_source=None):
        try:
            self.assets = {}
            self.playlists = []
            if as_asset:
                assets_paths = as_asset['assets_paths']
                for asset in assets_paths:
                    with requests.get(asset, verify=False) as remote_file:
                        self.assets = {**self.assets, **yaml.load(remote_file.content)}
            if as_source:
                source_path = as_source['source_path']
                constructors = as_source['construct']
                name = as_source['namespace']
                with requests.get(source_path, verify=False) as remote_file:
                    self.sources = get_sources_list(remote_file.content, constructors)
                    self.assets = sources_to_asset(name, self.sources)

        except ConnectionError:
            return {'error': 'URL not available'}
