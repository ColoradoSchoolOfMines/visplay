import os
import yaml

# Every source needs to say whether the files it gets survive after an error,
# how to get an asset file, and how to get a playlist file.


# Local Source
def localSource():
    source = {"survive": True,
              "getAsset": getAssetList,
              "getPlaylist": getplaylistList}
    return source


def getAssetList(timeStamp):
    return getLocalYaml("assets", timeStamp)


def getplaylistList(timeStamp):
    return getLocalYaml("playlist", timeStamp)


def getLocalYaml(path, timeStamp):
    try:
        localTimeStamp = os.path.getmtime(path + ".yaml")
        with open(path + ".yaml") as assets:
            if localTimeStamp > timeStamp:
                return {path: yaml.load(assets)}
            else:
                return {"error": "Old"}
    except OSError:
        return {"error": "An error parsing the yaml"}
