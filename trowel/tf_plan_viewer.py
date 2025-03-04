import json
from textual.app import App, ComposeResult
from textual.widgets import Footer, Header, Tree

class TfPlanTree(Tree):
  BINDINGS = [
    ("j", "cursor_down", "Cursor Down"),
    ("k", "cursor_up", "Cursor Up"),
    ("l", "select_cursor", "Select"),
    ("h", "cursor_parent", "Cursor to parent")
  ]

  show_root = False
  show_guides = True
  guide_depth = 4

  def __init__(self, description, json_file_path):
    super().__init__(description)
    self.json_data = self._load_json_file(json_file_path)
    self._build_tree()

  def _load_json_file(self, file_path: str):
    with open(file_path, "r", encoding="utf-8") as file:
      return json.load(file)

  def _build_tree(self):
    replaced = self.root.add("[bold green]Added[/bold green]")
    modified = self.root.add("[bold yellow]Modified[/bold yellow]")
    replaced = self.root.add("[bold red]Replaced[/bold red]")
    removed = self.root.add("[bold red]Removed[/bold red]")

    for resource in self.json_data["resource_changes"]:
        actions = resource["change"]["actions"]
        action_reason = "action_reason" in resource and resource["action_reason"]
        if len(actions) == 1 and actions[0] == "no-op":
            continue # no changes to make for this resource
        if action_reason == "replace_because_cannot_update":
          addr = replaced.add(resource["address"])

          for label in ["before", "after", "after_unknown", "before_sensitive", "after_sensitive"]:
            section = addr.add(label)
            entries = resource["change"][label].items()
            for k, v in entries:
              l = section.add(k)
              l.add_leaf(str(v))
        else:
            raise Exception(f"Invalid action_reason: {action_reason}")

class TfPlanViewerApp(App):
    BINDINGS = [("d", "toggle_dark", "Toggle dark mode")]

    def __init__(self, json_file_path):
        super().__init__()
        self.json_file_path = json_file_path

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()
        yield Footer()
        yield TfPlanTree("Plan Output", self.json_file_path)

    def action_toggle_dark(self) -> None:
        """An action to toggle dark mode."""
        self.theme = (
            "textual-dark" if self.theme == "textual-light" else "textual-light"
        )
