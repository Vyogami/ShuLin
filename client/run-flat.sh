#!/usr/bin/bash

BUILDDIR=$HOME/tmp/.flatpak/repo
STATEDIR=$HOME/tmp/.flatpak/flatpak-builder
APPID=com.github.vyogami.shulin
APP_CONF=$APPID.yml
APPBIN=shulin
mkdir -p $BUILDDIR

flatpak-builder --force-clean --state-dir=$STATEDIR $BUILDDIR $APP_CONF
flatpak-builder --run $BUILDDIR $APP_CONF $APPBIN
