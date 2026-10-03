#!/bin/sh
export GSK_RENDERER=cairo
if [ "${XDG_SESSION_TYPE:-}" = x11 ]; then export GDK_BACKEND=x11; fi
cd /home/fedora/RustroverProjects/lcars-desktop
exec ./target/debug/lcars-desktop
