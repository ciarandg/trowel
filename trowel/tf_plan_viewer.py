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
        change_dict = resource["change"]
        actions = change_dict["actions"]
        action_reason = "action_reason" in resource and resource["action_reason"]
        if len(actions) == 1 and actions[0] == "no-op":
          continue # no changes to make for this resource
        if action_reason == "replace_because_cannot_update":
          addr = replaced.add(resource["address"])

          # get all field names
          field_dict_keys = ["before", "after", "after_unknown", "before_sensitive", "after_sensitive"];
          field_name_lists = [list(change_dict[k].keys()) for k in field_dict_keys]
          field_names = sorted(list(set([x for xs in field_name_lists for x in xs])))

          changes = {f: {} for f in field_names}

          for f, change in changes.items():
            section = addr.add(f)
            before_val = change_dict["before"].get(f)
            before_sensitive_val = change_dict["before_sensitive"].get(f)
            before_text = "Before: "
            if not (before_val is None):
                before_text += json.dumps(before_val)
            elif not (before_sensitive_val is None):
                before_text += f"{json.dumps(before_sensitive_val)} (sensitive)"
            else:
                before_text += json.dumps(None)

            after_val = change_dict["after"].get(f)
            after_sensitive_val = change_dict["after_sensitive"].get(f)
            after_unknown_val = change_dict["after_unknown"].get(f)
            after_text = "After: "
            if not after_val is None:
                after_text += json.dumps(after_val)
            elif not after_sensitive_val is None:
                after_text += json.dumps(after_sensitive_val)
            elif not after_unknown_val is None:
                after_text += "(known after apply)"
            else:
                after_text += json.dumps(None)

            section.add_leaf(before_text)
            section.add_leaf(after_text)
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
