import os
from pathlib import Path

import requests
import uri
import yaml

from visplay.setup_sources import get_sources_list, sources_to_asset

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


class Source:
    """Root source class. Any source should override the __init__ function to
    do the loading of the source.
    """

    def __init__(self, name, uri: uri.URI, is_import=False):
        """Initialize a source.

        Arguments:
        """
        self.assets = {}
        self.sources = []
        self.layout = None
        self.is_import = is_import

    def __repr__(self):
        return f'<{type(self).__name__} assets={self.assets} sources={self.sources}>'


class HTTPSource(Source):
    def __init__(self, name, uri: uri.URI, is_import=False):
        super().__init__(name, uri, is_import=is_import)
        try:
            if self.is_import:
                with requests.get(uri.base, verify=False) as remote_file:
                    self.sources = get_sources_list(remote_file.content)
                    self.assets = sources_to_asset(name, self.sources)
                    self.layout = None
            else:
                with requests.get(uri.base, verify=False) as remote_file:
                    self.assets = yaml.load(remote_file.content)

        except ConnectionError:
            return {'error': 'URL not available'}


class PathSource(Source):
    """Allow users to specify a path as a source.

    If the path is a directory, it will load all ``*.sources.yaml`` files as
    sources, and all other ``.yaml`` files as assets.  All non-YAML files will
    be loaded as assets asset with the filename as the name of the asset.

    If the path is a file, it will be added as an asset with the filename as
    the name of the asset.
    """

    def __init__(self, name, uri: uri.URI, is_import=False):
        super().__init__(name, uri, is_import=is_import)
        path = Path(uri.path)

        if not os.path.exists(path):
            raise Exception(f'{path} does not exist.')

        for file in (os.listdir(path) if os.path.isdir(path) else [path]):
            file_path = path.joinpath(file)

            if file_path.suffix == '.yaml':
                if self.is_import and '.sources' in file_path.suffixes:
                    with open(file_path) as source_file:
                        # Recursively discover all sources
                        self.sources += get_sources_list(source_file)

                        # Namespace the assets
                        self.assets.update(
                            sources_to_asset(name, self.sources))
                        self.layout = None
                else:
                    self.assets.update(get_local_yaml(file_path))
            else:
                self.assets[file_path.name] = str(file_path)


# A list of sources following a basic interface.
source_constructors = {
    'file': PathSource,
    'http': HTTPSource,
    'https': HTTPSource,
    'path': PathSource,
}
