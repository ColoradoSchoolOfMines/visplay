# Visplay
This project's goal is to make the TV's in the foyer of the buildings of Mines
display media of ACM's projects. The project uses MPV and python.

## Features
Stream videos and open media files using an `mpv` implementation.

## Documentation
Under Construction

## Configuration
Visplay by default looks in $XDG_CONFIG_HOME, and the $HOME/.config for visplay.yaml.
This is the main local configuration and points to any number of assets.yaml files and playlists.yaml files (whether local or remote)

See visplay.yaml.example, assets.yaml.example, and playlists.yaml.example for example configurations.

To create configs:

    cp 


## Dependencies

libmpv / mpv (depends on distro) 

youtube-dl

## Installation
In the cloned visplay repo run:

    pip3 install . --user

then you can just run `visplay`.

If you have already installed, add `--update` to the `pip install` command.

## Developing Locally

    install dependencies listed in "Dependencies"

    git clone https://github.com/ColoradoSchoolOfMines/visplay.git
    cd visplay
    pip3 install -e . --user

    create necessary config files as described in "Configuration"

    python3 visplay/__main__.py
