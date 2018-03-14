import yaml


def get_sources_list(sources_path, source_constructors):
    all_sources = []

    with open(sources_path) as sources_file:
        config = yaml.load(sources_file)

    config.sort(key=lambda x: x['priority'])

    # calls the corresponding source constructor (LocalSource/HTTPSource)
    # with the arguments provided, then appends to the list
    for source in config:
        source_type = source['type']
        args = source['args']
        new_source = source_constructors[source_type](**args)
        all_sources.append((new_source, source['priority']))

    return all_sources
