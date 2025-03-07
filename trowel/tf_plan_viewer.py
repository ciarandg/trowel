from enum import Enum
import json
from rich.text import Text
from textual.app import App, ComposeResult
from textual.widgets import Footer, Header, Label, Rule, Tree


class Verbs(Enum):
    CREATE = {"color": "green", "past_tense": "created"}
    UPDATE = {"color": "yellow", "past_tense": "updated"}
    REPLACE = {"color": "purple", "past_tense": "replaced"}
    DESTROY = {"color": "red", "past_tense": "destroyed"}


class Parser:
    def __init__(self, json_data):
        self.json_data = json_data

    def _all_field_names(self, resource):
        keys = [
            "before",
            "after",
            "after_unknown",
            "before_sensitive",
            "after_sensitive",
        ]
        entries = [resource["change"].get(k) for k in keys]
        filtered = [e for e in entries if e != False and e != None]
        field_name_lists = [list(e.keys()) for e in filtered]
        return sorted(list(set([x for xs in field_name_lists for x in xs])))

    def field_before_after(self, resource, field_name):
        out = []
        change_dict = resource["change"]
        before = change_dict.get("before")
        before_val = (
            before.get(field_name) if before else None
        )  # before is None for create operations
        before_sensitive = change_dict.get("before_sensitive")
        before_sensitive_val = (
            before_sensitive.get(field_name) if before_sensitive else None
        )  # before_sensitive is None for create operations
        before_text = "Before: "
        if not (before_val is None):
            before_text += json.dumps(before_val)
        elif not (before_sensitive_val is None):
            before_text += f"{json.dumps(before_sensitive_val)} (sensitive)"
        else:
            before_text += json.dumps(None)

        after = change_dict.get("after")
        after_val = (
            after.get(field_name) if after else None
        )  # after is None for destroy operations
        after_sensitive = change_dict.get("after_sensitive")
        after_sensitive_val = (
            after_sensitive.get(field_name) if after_sensitive else None
        )  # after_sensitive is None for destroy operations
        after_unknown_val = change_dict["after_unknown"].get(field_name)
        after_text = "After: "
        if not after_val is None:
            after_text += json.dumps(after_val)
        elif not after_sensitive_val is None:
            after_text += json.dumps(after_sensitive_val)
        elif not after_unknown_val is None:
            after_text += "(known after apply)"
        else:
            after_text += json.dumps(None)

        out.append(before_text)
        out.append(after_text)
        return out

    def _resource_verb(self, resource):
        actions = resource["change"]["actions"]
        if actions == ["no-op"]:
            return None  # no changes to make for this resource
        elif actions == ["create"]:
            return Verbs.CREATE
        elif actions == ["update"]:
            return Verbs.UPDATE
        elif actions == ["delete"]:
            return Verbs.DESTROY
        elif actions == ["delete", "create"]:
            return Verbs.REPLACE
        else:
            raise Exception(f"Invalid resource actions array:", actions)

    def _resource_label(self, resource):
        verb = self._resource_verb(resource)
        label = Text(resource["address"], style=f"bold {verb.value['color']}")
        label.append(f" will be {verb.value['past_tense']}", style="default")
        return label

    def parse(self):
        """Convert raw TF plan JSON to a diff structure"""
        out = {}

        for resource in self.json_data["resource_changes"]:
            verb = self._resource_verb(resource)
            if not verb:
                continue  # resource is no-op
            label = self._resource_label(resource)
            resource_entry = out.setdefault(label.markup, {})
            for f in self._all_field_names(resource):
                resource_entry[f] = self.field_before_after(resource, f)
        return out


class TfPlanTree(Tree):
    BINDINGS = [
        ("j", "cursor_down", "Cursor Down"),
        ("k", "cursor_up", "Cursor Up"),
        ("l", "select_cursor", "Select"),
        ("h", "cursor_parent", "Cursor to parent"),
    ]

    show_root = False
    show_guides = True
    guide_depth = 4

    def __init__(self, description, json_file_path):
        super().__init__(description)
        json_data = self._load_json_file(json_file_path)
        parser = Parser(json_data)
        self._build_tree(parser.parse(), self.root)

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
            for item in data:
                node.add_leaf(str(item))
        else:
            node.add_leaf(str(data))


class ExperimentalWarning(Label):
    def __init__(self):
        text = Text(
          "WARNING: This app is experimental and untested. Do not trust its output!",
          style="red bold underline"
        )
        super().__init__(text)
        self.styles.padding = 1


class TfPlanViewerApp(App):
    BINDINGS = [("d", "toggle_dark", "Toggle dark mode")]

    def __init__(self, json_file_path, hide_experimental_warning):
        super().__init__()
        self.json_file_path = json_file_path
        self.hide_experimental_warning = hide_experimental_warning

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()
        if not self.hide_experimental_warning:
            yield ExperimentalWarning()
        yield Footer()
        yield TfPlanTree("Plan Output", self.json_file_path)

    def action_toggle_dark(self) -> None:
        """An action to toggle dark mode."""
        self.theme = (
            "textual-dark" if self.theme == "textual-light" else "textual-light"
        )
