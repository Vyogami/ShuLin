from gi.repository import Gtk, Gio, Adw, GLib

from shulin.const import Constants
from shulin.functions import get_action_row

import requests
import json

def post_request(url, body, label="API"):
    headers = {
        "Content-Type": "application/json"
    }
    response = requests.post(url, headers=headers, data=body)
    if response.status_code == 200:
        print(f"{label} request successful")
    else:
        print(f"{label} request failed")



@Gtk.Template(resource_path=f'{Constants.PATHID}/ui/main.ui')
class MainWindow(Adw.ApplicationWindow):
    __gtype_name__ = "MainWindow"

    main_content = Gtk.Template.Child()
    flap = Gtk.Template.Child()
    stack = Gtk.Template.Child()
    stack_switch = Gtk.Template.Child()
    
    # Page1 widgets
    page1_box = Gtk.Template.Child()
    page1_grp1 = Gtk.Template.Child()

    def __init__(self, **kwargs):
        Adw.ApplicationWindow.__init__(self, **kwargs)
        self.style_manager = Adw.StyleManager().get_default()
        print(self.style_manager.get_system_supports_color_schemes())
        self.style_manager.set_color_scheme(Adw.ColorScheme.PREFER_LIGHT)

        # # setup menu actions
        self.create_action('preferences', self.menu_handler)
        self.create_action('about', self.menu_handler)
        self.create_action('quit', self.menu_handler)
        self.add_page1()
        self.css_provider = self.load_css()
        self.add_custom_styling(self.main_content)
        

    def add_page1(self):
        def ssh_switch_callback(switch, gparam):
            url = f"{Constants.BASE_URL}/ssh"
            if switch.get_active():
                data = {
                    "toggle": True
                }
            else:
                data = {
                    "toggle": False
                }
            body = json.dumps(data)
            post_request(url, body)
        row_ssh = get_action_row("Secure socket shell(SSH)", "SSH is a network protocol that provides a secure way to access and manage network services over an unsecured network, ensuring data confidentiality and integrity.", "find-location-symbolic", switch_callback=ssh_switch_callback)
        self.page1_grp1.add(row_ssh)

        def usb_switch_callback(switch, gparam):
            url = f"{Constants.BASE_URL}/usb"
            if switch.get_active():
                data = {
                    "toggle": True
                }
            else:
                data = {
                    "toggle": False
                }
            body = json.dumps(data)
            post_request(url, body)
        row_usb = get_action_row("USB Device Control", "USB (Universal Serial Bus) Device Control allows you to manage and secure USB ports on your system, regulating the connection of external USB devices.", "usb-symbolic", switch_callback=usb_switch_callback)
        self.page1_grp1.add(row_usb)

    @Gtk.Template.Callback()
    def on_color_switch(self, *args):
        if self.style_manager.get_dark():
            self.style_manager.set_color_scheme(Adw.ColorScheme.FORCE_LIGHT)
        else:
            self.style_manager.set_color_scheme(Adw.ColorScheme.FORCE_DARK)


    @Gtk.Template.Callback()
    def on_flap_toggled(self, widget):
        self.flap.set_reveal_flap(not self.flap.get_reveal_flap())

    def on_button_clicked(self, widget):
        label = widget.get_label()
        print(f'Button {label} Pressed')
        

    def menu_handler(self, action, state):
        """ Callback for  menu actions"""
        name = action.get_name()
        print(f'active : {name}')
        if name == 'quit':
            self.close()

    def load_css(self):
        """create a provider for custom styling"""
        css_provider = Gtk.CssProvider()
        css_path = f'{Constants.PATHID}/css/main.css'
        try:
            css_provider.load_from_resource(resource_path=css_path)
        except GLib.Error as e:
            print(f"Error loading CSS : {e} ")
            return None
        print(f'loading custom styling from resource: {css_path}')
        # print(css_provider.to_string())
        return css_provider

    def _add_widget_styling(self, widget):
        if self.css_provider:
            # print(f'Adding style to : {widget.props.css_name}')
            context = widget.get_style_context()
            context.add_provider(
                self.css_provider, Gtk.STYLE_PROVIDER_PRIORITY_USER)

    def add_custom_styling(self, widget):
        self._add_widget_styling(widget)
        # iterate children recursive
        for child in widget:
            self.add_custom_styling(child)

    def create_action(self, name, callback):
        """ Add an Action and connect to a callback """
        action = Gio.SimpleAction.new(name, None)
        action.connect("activate", callback)
        self.add_action(action)
