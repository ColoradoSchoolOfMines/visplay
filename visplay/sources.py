import os
from pathlib import Path

import requests
import uri
import yaml

from visplay.setupConfig import get_sources_list, sources_to_asset

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
    def __init__(self, constructors, name, uri: uri.URI):
        self.survive = True
        if constructors:
            source_path = uri.path
            with open(source_path) as source_file:
                # Recursively discover all sources
                self.sources = get_sources_list(source_file, constructors)
                # Namespace the assets
                self.assets = sources_to_asset(name, self.sources)
        else:
            self.assets = get_local_yaml(uri.path)


class HTTPSource:
    def __init__(self, constructors, name, uri: uri.URI):
        try:
            self.assets = {}
            if constructors:
                with requests.get(uri.base, verify=False) as remote_file:
                    self.sources = get_sources_list(remote_file.content,
                                                    constructors)
                    self.assets = sources_to_asset(name, self.sources)
            else:
                with requests.get(uri.path, verify=False) as remote_file:
                    self.assets = yaml.load(remote_file.content)

        except ConnectionError:
            return {'error': 'URL not available'}


class PathSource:
    """Allow users to specify a path as a source.

    If the path is a directory, it will check for any ``.yaml`` file and load
    it as an asset. All other files will be loaded as an asset with the
    filename as the name of the asset.

    If the path is a file, it will be added as an asset with the filename as
    the name of the asset.
    """

    def __init__(self, constructors, name, uri: uri.URI):
        self.assets = {}

        path = Path(uri.path)

        if os.path.isdir(path):
            for file in os.listdir(path):
                file_path = path.joinpath(file)

                if file_path.suffix == '.yaml':
                    self.assets.update(get_local_yaml(file_path))
                else:
                    self.assets[file_path.name] = str(file_path)
        elif os.path.isfile(path):
            # Load the file as an asset
            self.assets[path.name] = str(path)
        else:
            raise Exception(f'{path} does not exist.')
