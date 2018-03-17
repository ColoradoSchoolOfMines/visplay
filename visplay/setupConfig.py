import yaml


def get_sources_list(sources_path, source_constructors):
    all_sources = []

    with open(sources_path) as sources_file:
        config = yaml.load(sources_file)

    # Handle imports
    if 'import' in config:
        for source in config['import']:
            source_type = source['type']
            args = {'as_source': source['args']}
            args['as_source']['construct'] = source_constructors
            new_source = source_constructors[source_type](**args)
            if 'priority' in source:
                new_source.priority = source['priority']
            else:
                new_source.priority = 0
            all_sources.append(new_source)

    if 'add' in config:
        # calls the corresponding source constructor (LocalSource/HTTPSource)
        # with the arguments provided, then appends to the list
        for source in config['add']:
            source_type = source['type']
            args = {'as_asset': source['args']}
            new_source = source_constructors[source_type](**args)
            if 'priority' in source:
                new_source.priority = source['priority']
            else:
                new_source.priority = 0
            all_sources.append(new_source)

    return all_sources

def sources_to_asset(name, sources):
    assets = {}
    for source in sources:
        if source.assets:
            for asset in source.assets:
                assets[name + ":" + asset] = source.assets[asset]
    return assets

def sources_to_play(name, sources):
    playlists = []
    for source in sources:
        if source.playlists:
            for asset in source.playlists:
                playlists.append(name + ":" + asset)
    return playlists
