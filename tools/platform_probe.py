"""Read-only capability probe; no settings or windows are changed."""
import gi
import os
import subprocess
import shutil

gi.require_version('Gtk', '4.0')
from gi.repository import Gtk, Gdk
print('GTK', Gtk.get_major_version(), Gtk.get_minor_version(), Gtk.get_micro_version())
print('GTK foreign window embedding Socket API:', hasattr(Gtk, 'Socket'))
print('GTK IM context:', hasattr(Gtk, 'IMMulticontext'))
print('GTK Accessible:', hasattr(Gtk, 'Accessible'))
Gtk.init()
display = Gdk.Display.get_default()
print('Display:', display.get_name() if display else None)
if display:
    print('Backend:', display.__class__.__name__)
    seat = display.get_default_seat()
    print('Seat capabilities:', seat.get_capabilities())
    for device in seat.get_devices(Gdk.SeatCapabilities.ALL):
        print('Device:', device.get_name(), 'source:', device.get_source())
for key in ['screen-keyboard-enabled']:
    result = subprocess.run(['gsettings', 'get', 'org.gnome.desktop.a11y.applications', key], capture_output=True, text=True)
    print('GNOME', key, result.stdout.strip() or result.stderr.strip())
print('External window inspection tool xprop:', bool(shutil.which('xprop')))
print('Window manager control tool wmctrl:', bool(shutil.which('wmctrl')))
print('Session metadata:')
subprocess.run(['loginctl','show-user','fedora','-p','Sessions','-p','Display'],check=False)

subprocess.run(['loginctl','show-session','2','-p','Type','-p','Desktop','-p','Display'],check=False)
if display and 'X11' in display.__class__.__name__:
    result = subprocess.run(['xprop','-root','_NET_SUPPORTED'],capture_output=True,text=True)
    for capability in ['_NET_CLIENT_LIST','_NET_ACTIVE_WINDOW','_NET_MOVERESIZE_WINDOW']:
        print('EWMH',capability,capability in result.stdout)
