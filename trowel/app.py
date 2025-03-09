from enum import Enum
import json
from rich.text import Text
from textual.app import App, ComposeResult
from textual.widgets import Footer, Header, Label, Rule, Tree


class CustomTree(Tree):
    show_root = False
    def __init__(self, description):
        super().__init__(description)
        self.root.add(Text("this is a darker yellow", style="bold yellow").markup)
        self.root.add(Text("this is a darker yellow", style="bold yellow").markup)
        self.root.add(Text("this is a darker yellow", style="bold yellow").markup)


class MyApp(App):
    BINDINGS = [("d", "toggle_dark", "Toggle dark mode")]

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()
        yield CustomTree("description")
        yield Label(Text("this is a brighter yellow!", style="bold yellow"))
        yield Footer()

    def action_toggle_dark(self) -> None:
        """An action to toggle dark mode."""
        self.theme = (
            "textual-dark" if self.theme == "textual-light" else "textual-light"
        )
