import json
from textual.app import App, ComposeResult
from textual.widgets import Footer, Header, Tree

class JsonTree(Tree):
  BINDINGS = [
    ("j", "cursor_down", "Cursor Down"),
    ("k", "cursor_up", "Cursor Up"),
    ("l", "select_cursor", "Select"),
    ("h", "cursor_parent", "Cursor to parent")
  ]

  def __init__(self, description, file_path):
    super().__init__(description)
    data = self._load_json_file(file_path)
    self.root.expand()
    self._build_tree(data, self.root)

  def _load_json_file(self, file_path: str):
    with open(file_path, "r", encoding="utf-8") as file:
      return json.load(file)

  def _build_tree(self, data, node):
      """Recursively build a tree structure from a nested dictionary."""
      if isinstance(data, dict):
          for key, value in data.items():
              subtree = node.add(key)
              self._build_tree(value, subtree)
      elif isinstance(data, list):
          for index, item in enumerate(data):
              subtree = node.add(f"[{index}]")
              self._build_tree(item, subtree)
      else:
          node.add_leaf(str(data))

class JsonViewerApp(App):
    BINDINGS = [("d", "toggle_dark", "Toggle dark mode")]

    def __init__(self, json_file_path):
        super().__init__()
        self.json_file_path = json_file_path

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()
        yield Footer()
        yield JsonTree("Plan Output", self.json_file_path)

    def action_toggle_dark(self) -> None:
        """An action to toggle dark mode."""
        self.theme = (
            "textual-dark" if self.theme == "textual-light" else "textual-light"
        )
