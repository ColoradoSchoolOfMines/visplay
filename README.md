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

    mkdir ~/.config/visplay
    cp config.yaml.example ~/.config/visplay/config.yaml
    cp sources.yaml.example ~/.config/visplay/sources.yaml
    cp assets.yaml.example ~/.config/visplay/assets.yaml
    cp playlists.yaml.example ~/.config/visplay/playlists.yaml

    edit ~/.config/visplay.yaml to point to assets.yaml and playlists.yaml

    edit assets.yaml to point to valid assets

## Dependencies

python3

python3-pip (pip3)

libmpv / mpv (depends on distro, some bundle them)

youtube-dl

qt5, qmake-qt5

boost-python3

## Installation
In the cloned visplay repo run:
    
    #install dependencies listed in "Dependencies"

    git clone https://github.com/ColoradoSchoolOfMines/visplay.git
    cd visplay
    pip3 install -e . --user

    #create necessary config files as described in "Configuration"

then you can just run `visplay`.

If you have already installed, add `--update` to the `pip install` command.


