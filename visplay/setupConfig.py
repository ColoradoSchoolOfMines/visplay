import yaml


def get_source_list(config_file, source_constructors):
    all_sources = []
    config = yaml.load(config_file)
    config.sort(key=lambda x: x['priority'])
    for source in config:
        source_type = source['type']
        args = source['args']
        all_sources.append(source_constructors[source_type](**args))
    return all_sources
