import yaml
import requests
import uri
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

    def __init__(self, constructors, name, path: uri.URI):
        self.survive = True
        if constructors:
            source_path = path.path
            with open(source_path) as source_file:
                # Recursively discover all sources
                self.sources = get_sources_list(source_file, constructors)
                # Namespace the assets
                self.assets = sources_to_asset(name, self.sources)
        else:
            self.assets = {}
            self.assets = get_local_yaml(path.path)


class HTTPSource:

    def __init__(self, constructors, name, path: uri.URI):
        try:
            self.assets = {}
            if constructors:
                with requests.get(path.base, verify=False) as remote_file:
                    self.sources = get_sources_list(remote_file.content, constructors)
                    self.assets = sources_to_asset(name, self.sources)
            else:
                with requests.get(path.path, verify=False) as remote_file:
                    self.assets = yaml.load(remote_file.content)

        except ConnectionError:
            return {'error': 'URL not available'}
