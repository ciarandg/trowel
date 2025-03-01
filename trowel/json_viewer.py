import json
from textual.app import App, ComposeResult
from textual.widgets import Footer, Header, Tree

def load_json_file(file_path: str):
  with open(file_path, "r", encoding="utf-8") as file:
    return json.load(file)

def build_json_tree(data, node):
    """Recursively build a tree structure from a nested dictionary."""
    if isinstance(data, dict):
        for key, value in data.items():
            subtree = node.add(key)
            build_json_tree(value, subtree)
    elif isinstance(data, list):
        for index, item in enumerate(data):
            subtree = node.add(f"[{index}]")
            build_json_tree(item, subtree)
    else:
        node.add_leaf(str(data))

def json_tree(file_path: str):
    data = load_json_file(file_path)
    tree = Tree("Plan Output")
    tree.root.expand()
    build_json_tree(data, tree.root)
    return tree

class JsonViewerApp(App):
    BINDINGS = [("d", "toggle_dark", "Toggle dark mode")]

    def __init__(self, json_file_path):
        super().__init__()
        self.json_file_path = json_file_path

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()
        yield Footer()

        yield json_tree(self.json_file_path)

    def action_toggle_dark(self) -> None:
        """An action to toggle dark mode."""
        self.theme = (
            "textual-dark" if self.theme == "textual-light" else "textual-light"
        )
