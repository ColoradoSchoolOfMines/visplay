import os
import yaml
import requests

# Every source needs to say whether the files it gets survive after an error,
# how to get an asset file, and how to get a playlist file.


class LocalSource:

    def __init__(self):
        self.survive = True

    def get_assets(self, timeStamp):
        return self.getLocalYaml('assets', timeStamp)

    def get_playlist(self, timeStamp):
        return self.getLocalYaml('playlist', timeStamp)

    def getLocalYaml(self, path, timeStamp):
        try:
            localTimeStamp = os.path.getmtime(path + '.yaml')
            with open(path + '.yaml') as assets:
                if localTimeStamp > timeStamp:
                    return {path: yaml.load(assets)}
                else:
                    return {'error': 'Old'}
        except OSError:
            return {'error': 'An error parsing the yaml'}


class HTTPSource:

    def __init__(self, urls):

        with requests.get(urls['assets'], verify=False) as remote_file:
            self.asset_yaml = yaml.load(remote_file.content)

        with requests.get(urls['playlist'], verify=False) as remote_file:
            self.playlist_yaml = yaml.load(remote_file.content)
