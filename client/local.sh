#!/usr/bin/bash
echo "Uninstalling"
sudo rm -rf /usr/lib/python3.11/site-packages/shulin
sudo rm -rf /usr/share/shulin/
sudo rm /usr/bin/shulin 
echo "Rebuilding"
rm -rf _build/
meson --prefix /usr _build && cd _build
meson compile
echo "Installing"
sudo meson install
cd ..
/usr/bin/shulin
