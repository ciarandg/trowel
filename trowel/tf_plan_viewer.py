from enum import Enum
import json
from rich.text import Text
from textual.app import App, ComposeResult
from textual.widgets import Footer, Header, Label, Rule, Static, TextArea, Tree


class Verbs(Enum):
    CREATE = {"color": "green", "past_tense": "created", "icon": "+", "sort_key": 0}
    UPDATE = {"color": "yellow", "past_tense": "updated", "icon": "~", "sort_key": 3}
    REPLACE = {"color": "purple", "past_tense": "replaced", "icon": "r", "sort_key": 2}
    DESTROY = {"color": "red", "past_tense": "destroyed", "icon": "-", "sort_key": 1}
    READ = {"color": "cyan", "past_tense": "read", "icon": "?", "sort_key": 4}


class Parser:
    def __init__(self, json_plan):
        self.json_plan = json_plan

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

    def _field_before(self, resource, field_name):
        change_dict = resource["change"]
        before = change_dict.get("before")
        before_val = (
            before.get(field_name) if before else None
        )  # before is None for create operations
        before_sensitive = change_dict.get("before_sensitive")
        before_sensitive_val = (
            before_sensitive.get(field_name) if before_sensitive else None
        )  # before_sensitive is None for create operations
        out = {
          "value": None,
          "sensitive": False,
        }
        if not (before_val is None):
            out["value"] = before_val
        elif not (before_sensitive_val is None):
            out["value"] = before_sensitive_val
            out["sensitive"] = True
        return out

    def _field_after(self, resource, field_name):
        change_dict = resource["change"]
        after = change_dict.get("after")
        after_val = (
            after.get(field_name) if after else None
        )  # after is None for destroy operations
        after_sensitive = change_dict.get("after_sensitive")
        after_sensitive_val = (
            after_sensitive.get(field_name) if after_sensitive else None
        )  # after_sensitive is None for destroy operations
        after_unknown_val = change_dict["after_unknown"].get(field_name)
        out = {
            "value": None,
            "known_after_apply": False,
        }
        if not after_val is None:
            out["value"] = after_val
        elif not after_sensitive_val is None:
            out["value"] = after_sensitive_val
        elif not after_unknown_val is None:
            out["known_after_apply"] = True
        return out

    def _field_before_after(self, resource, field_name):
        change_dict = resource["change"]
        before = self._field_before(resource, field_name)
        before_text = "(sensitive value)" if before["sensitive"] else json.dumps(before["value"])
        after = self._field_after(resource, field_name)
        after_text = "(known after apply)" if after["known_after_apply"] else json.dumps(after["value"])
        return [before_text, after_text]

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
        elif sorted(actions) == ["create", "delete"]:
            return Verbs.REPLACE
        elif actions == ["read"]:
            return Verbs.READ
        else:
            raise Exception(f"Invalid resource actions array:", actions)

    def _resource_label(self, resource):
        verb = self._resource_verb(resource)
        label = Text(resource["address"], style=f"bold {verb.value['color']}")
        label.append(f" will be {verb.value['past_tense']}", style="default")
        return label

    def parse_counts(self):
        """Convert raw TF plan JSON to modification counts by verb"""
        out = {}
        for resource in self.json_plan["resource_changes"]:
            verb = self._resource_verb(resource)
            if verb is None:
                continue
            out.setdefault(verb, 0)
            out[verb] += 1
        return out

    def parse_diff(self):
        """Convert raw TF plan JSON to a diff structure"""
        out = {}

        for resource in self.json_plan["resource_changes"]:
            verb = self._resource_verb(resource)
            if not verb:
                continue  # resource is no-op
            label = self._resource_label(resource)
            resource_entry = out.setdefault(label.markup, [])
            unchanged_count = 0
            for f in self._all_field_names(resource):
                [before_text, after_text] = self._field_before_after(resource, f)
                if before_text != after_text:
                    resource_entry.append(f"{f} {before_text} -> {after_text}")
                else:
                    unchanged_count += 1
            resource_entry.append(f"{unchanged_count} unchanged")
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

    def __init__(self, description, json_plan, id=None):
        super().__init__(description, id=id)
        parser = Parser(json_plan)
        self._build_tree(parser.parse_diff(), self.root)

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


class Summary(Label):
    def __init__(self, json_plan, id=None):
        parser = Parser(json_plan)
        counts = parser.parse_counts()
        sorted_counts = sorted(counts.items(), key=lambda pair: pair[0].value['sort_key'])
        text = Text()
        for index, (verb, count) in enumerate(sorted_counts):
            text.append(
                f"{verb.value['icon']}{count}",
                style=f"bold {verb.value['color']}"
            )
            if index < len(counts) - 1:
                text.append(" ")
        super().__init__(text.markup, id=id)


class TfPlanViewerApp(App):
    CSS_PATH = "app.tcss"
    BINDINGS = [("d", "toggle_dark", "Toggle dark mode"),
                ("v", "toggle_view", "Toggle view")]

    def __init__(self, json_plan, text_plan, hide_experimental_warning):
        super().__init__()

        self.tree_view = True
        self.add_class("treeview")

        self.json_plan = json_plan
        self.text_plan = text_plan
        self.hide_experimental_warning = hide_experimental_warning

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        if not self.hide_experimental_warning:
            yield ExperimentalWarning()
        yield TfPlanTree("Plan Output", self.json_plan, id="tree-plan")
        if self.text_plan:
            yield Static(Text(self.text_plan), id="text-plan")
        yield Summary(self.json_plan, id="summary")
        yield Footer()

    def action_toggle_dark(self) -> None:
        """An action to toggle dark mode."""
        self.theme = (
            "textual-dark" if self.theme == "textual-light" else "textual-light"
        )

    def action_toggle_view(self) -> None:
        """An action to toggle viewing mode."""
        self.tree_view = not self.tree_view
        if self.tree_view:
          self.remove_class("textview")
          self.set_focus(self.get_widget_by_id("tree-plan"))
        else:
          self.add_class("textview")
          self.set_focus(self.get_widget_by_id("text-plan"))
