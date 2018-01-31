import yaml


def get_source_list(config_file, source_constructors):
    all_sources = []
    config = yaml.load(config_file)
    print(source_constructors)
    config.sort(key=lambda x: x["priority"])
    for source in config:
        name = source["name"]
        args = source["args"]
        all_sources.append(source_constructors[name](args))
    return all_sources
