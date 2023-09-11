
from gi.repository import Adw
from gi.repository import Gtk

@Gtk.Template(resource_path='/com/github/vyogami/shulin/window.ui')
class ShulinWindow(Adw.ApplicationWindow):
    __gtype_name__ = 'ShulinWindow'

    label = Gtk.Template.Child()

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
