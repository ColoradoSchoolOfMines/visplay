import yaml


def get_source_list(config_file, source_constructors):
    all_sources = []
    config = yaml.load(config_file)
    print(source_constructors)
    config.sort(key=lambda x: x["priority"])
    for source in config:
        video_type = source["type"]
        args = source["args"]
        all_sources.append(source_constructors[video_type](args))
    return all_sources
