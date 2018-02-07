import os
import yaml
import requests

# Every source needs to say whether the files it gets survive after an error,
# how to get an asset file, and how to get a playlist file.


class LocalSource:

    def __init__(self, path):
        self.survive = True
        self.path = path
        self.assets = self.get_local_yaml('assets', 0)
        self.playlist = self.get_local_yaml('playlist', 0)

    def get_local_yaml(self, path, timeStamp):
        try:
            localTimeStamp = os.path.getmtime(self.path + path + '.yaml')
            with open(self.path + path + '.yaml') as assets:
                if localTimeStamp > timeStamp:
                    return yaml.load(assets)
                else:
                    return {'error': 'Old'}
        except OSError:
            return {'error': 'An error parsing the yaml'}


class HTTPSource:

    def __init__(self, urls):

        if 'assets' in urls:
            with requests.get(urls['assets'], verify=False) as remote_file:
                self.assets = yaml.load(remote_file.content)

        if 'playlists' in urls:
            with requests.get(urls['playlist'], verify=False) as remote_file:
                self.playlist = yaml.load(remote_file.content)
