import uri
import yaml


def get_sources_list(source_stream):
    from visplay.sources import source_constructors as sc
    all_sources = []

    sources_yaml = yaml.load(source_stream)

    # Handle imports
    if 'import' in sources_yaml and sources_yaml['import']:
        for name in sources_yaml['import']:
            source = sources_yaml['import'][name]
            path = uri.URI(source)
            new_source = sc[str(path.scheme)](name, path, is_import=True)
            all_sources.append(new_source)

    if 'add' in sources_yaml and sources_yaml['add']:
        # calls the corresponding source constructor (LocalSource/HTTPSource)
        # with the arguments provided, then appends to the list
        for source in sources_yaml['add']:
            path = uri.URI(source)
            new_source = sc[str(path.scheme)](source, path)
            all_sources.append(new_source)

    return all_sources


# Create the necessary namespaces
def sources_to_asset(name, sources):
    assets = {}
    for source in sources:
        for asset in source.assets:
            if type(source.assets[asset]) is list:
                source_asset = source.assets[asset]
                source_asset[:] = [name + ":" + item for item in source_asset]
            assets[name + ":" + asset] = source.assets[asset]
    return assets


def sources_to_play(name, sources):
    playlists = []
    for source in sources:
        if source.playlists:
            for asset in source.playlists:
                playlists.append(name + ":" + asset)
    return playlists
