#!/bin/zsh
pyinstaller fc.json.py
chmod +x dist/fc.json/fc.json
ln -fs $PWD/dist/fc.json/fc.json bin/fc.json