import sys
import gi

gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')

from gi.repository import Gtk, Gio, GLib, Adw

from shulin.window import MainWindow

APP_ID = 'com.github.vyogami.shulin'


class Application(Adw.Application):
    def __init__(self):
        Adw.Application.__init__(self, application_id=APP_ID,
                                 flags=Gio.ApplicationFlags.FLAGS_NONE)

    def do_activate(self):
        win = self.props.active_window
        if not win:
            win = MainWindow(application=self)
        win.present()


def main(version):
    app = Application()
    return app.run(sys.argv)
