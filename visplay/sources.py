import os
from pathlib import Path

import ipfsapi as ipfs
import requests
import uri
import yaml
from ipfsapi import exceptions

from visplay.config import Config
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
        self.is_import = is_import

    def __repr__(self):
        return f'<{type(self).__name__} assets={self.assets} sources={self.sources}>'

    def _from_stream(name, stream):
        sources = get_sources_list(stream)

        # Namespace the assets
        assets = sources_to_asset(name, sources)
        return (sources, assets)


class HTTPSource(Source):
    def __init__(self, name, uri: uri.URI, is_import=False):
        super().__init__(name, uri, is_import=is_import)
        try:
            if self.is_import:
                with requests.get(uri.base, verify=False) as remote_file:
                    self.sources, self.assets = self._from_stream(
                        name, remote_file.content)
            else:
                with requests.get(uri.path, verify=False) as remote_file:
                    self.assets = yaml.load(remote_file.content)

        except ConnectionError:
            return {'error': 'URL not available'}


class IPFSSource(Source):
    """Allow users to specify an IPFS hash as a source.

    If the source
    """

    def __init__(self, name, uri: uri.URI, is_import=False):
        super().__init__(name, uri, is_import=is_import)
        try:
            ipfs_config = Config.get('ipfs_config')
            ipfs_api = ipfs.connect(
                host=ipfs_config.get('host'),
                port=ipfs_config.get('port'),
            )
        except exceptions.ConnectionError as e:
            raise ConnectionError(f'Could not connect to IPFS.') from e

        try:
            file_info = ipfs_api.ls(uri.path)

            for obj in file_info.get('Objects'):
                print(obj)
                if len(obj.get('Links', [])) > 0:
                    for link in obj['Links']:
                        print(link)
                    # directory
                    pass
                else:
                    # file
                    file = ipfs_api.cat(uri.path).decode()
                    print(file)
        except exceptions.Error as e:
            print(e)


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
            raise IOError(f'{path} does not exist.')

        for file in (os.listdir(path) if os.path.isdir(path) else [path]):
            file_path = path.joinpath(file)

            if file_path.suffix == '.yaml':
                if self.is_import and '.sources' in file_path.suffixes:
                    with open(file_path) as source_file:
                        # Recursively discover all sources
                        sources, assets = self._from_stream(name, source_file)
                        self.sources += sources
                        self.assets.update(assets)
                else:
                    self.assets.update(get_local_yaml(file_path))
            else:
                self.assets[file_path.name] = str(file_path)


# A list of sources following a basic interface.
source_constructors = {
    'file': PathSource,
    'http': HTTPSource,
    'https': HTTPSource,
    'ipfs': IPFSSource,
    'path': PathSource,
}
