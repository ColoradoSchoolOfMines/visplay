# Visplay
This project's goal is to make the TV's in the foyer of the buildings of Mines
display media of ACM's projects. The project uses MPV and python.

## Features
Stream videos and open media files using an `mpv` implementation.

## Installation
In the cloned visplay repo run:
    
    #install dependencies listed in "Dependencies"

    git clone https://github.com/ColoradoSchoolOfMines/visplay.git
    cd visplay
    pip3 install -e . --user

    #create necessary config files as described in "Configuration"

then you can just run `visplay`.

If you have already installed, add `--update` to the `pip install` command.

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

libmpv / mpv (depends on distro) 

youtube-dl

## Documentation
### Yaml Configuration Layout
config.yaml: Points source info, potentailly in the future this can be used to configure other options

    sourcename: Directory to sources.yaml

sources.yaml: Points to local and networked asssest and playlist lists
    
    - name: local
      type: local
      args:
        assets_path: /home/user/development/ACM/visplay/assets.yaml
        playlists_path:/home/user/development/ACM/visplay/playlists.yaml
      priority: 8
    - name: my_server
      type: http
      args:
        assets_path: http://example.com/assets.yaml
        playlists_path: http://exmaple.com/playlists.yaml
      priority: 0

assets.yaml : Contains File pathes to assets

    TheAssetName: local path or url
    CoolestVidEver: https://www.youtube.com/watch?v=dQw4w9WgXcQ

playlists.yaml: Controls control flow for assets

    - CoolestVidEver
    - TheAssetName