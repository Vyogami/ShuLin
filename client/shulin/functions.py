from gi.repository import Gtk, Adw

def get_action_row(action_title, action_subtitle, action_icon):
    row = Adw.ActionRow()
    row.set_title(action_title)
    row.set_subtitle(f"{action_subtitle}")
    row.set_icon_name(f"{action_icon}")
    switch = Gtk.Switch()
    switch.props.halign = Gtk.Align.CENTER
    switch.props.valign = Gtk.Align.CENTER
    switch.props.hexpand = False
    switch.props.vexpand = False
    row.add_suffix(switch)
    return row
 