# Visplay
This project's goal is to make the TV's in the foyer of the buildings of Mines
display media of ACM's projects. The project uses MPV and python.

## Features
Stream videos and open media files using an `mpv` implementation.

## Documentation
Under Construction

## Configuration
Visplay by default looks in $XDG_CONFIG_HOME, and the $HOME/.config for visplay.conf.
This is the main local configuration and points to any number of assets.yaml files and playlists.yaml files (whether local or remote)

See visplay.yaml.example, assets.yaml.example, and playlists.yaml.example for example configurations.


## Dependencies
MPV
libmpv

## Installation
In the cloned visplay repo run:

    pip3 install . --user

then you can just run `visplay`.

If you have already installed, add `--update` to the `pip install` command.

## Developing Locally
Run:

    pip install -e . --user
    python3 visplay/__main__.py
