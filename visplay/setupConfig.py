import yaml


def get_source_list(config_file, source_constructors):
    all_sources = []
    config = yaml.load(config_file)
    config.sort(key=lambda x: x['priority'])
    for source in config:
        source_type = source['type']
        args = source['args']
        new_source = source_constructors[source_type](**args)
        all_sources.append((new_source, source['priority']))
    return all_sources
