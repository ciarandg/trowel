import json
from textual.app import App, ComposeResult
from textual.widgets import Footer, Header, Tree


class JsonTree(Tree):
    BINDINGS = [
        ("j", "cursor_down", "Cursor Down"),
        ("k", "cursor_up", "Cursor Up"),
        ("l", "select_cursor", "Select"),
        ("h", "cursor_parent", "Cursor to parent"),
    ]

    show_root = False
    show_guides = True
    guide_depth = 4

    def __init__(self, description):
        super().__init__(description)
        replaced = self.root.add("[bold green]Added[/bold green]")
        modified = self.root.add("[bold yellow]Modified[/bold yellow]")
        replaced = self.root.add("[bold red]Replaced[/bold red]")
        removed = self.root.add("[bold red]Removed[/bold red]")

        l1 = replaced.add("cloudflare_record.foo")
        leaves = [
            {
                "name": "content",
                "before": "",
                "after": "(known after apply)",
                "color": "green",
            },
            {
                "name": "created_on",
                "before": '"1970-01-01T00:00:00.0000000Z"',
                "after": "(known after apply)",
                "color": "yellow",
            },
            {
                "name": "hostname",
                "before": "foo.example.com",
                "after": "(known after apply)",
                "color": "yellow",
            },
            {
                "name": "id",
                "before": '"86e1f0f8-3902-440a-aa6e-beec3a30cb54"',
                "after": "(known after apply)",
                "color": "yellow",
            },
            {
                "name": "metadata",
                "before": "{}",
                "after": "(known after apply)",
                "color": "yellow",
            },
            {
                "name": "modified_on",
                "before": "1970-01-01T00:00:00.0000000Z",
                "after": "(known after apply)",
                "color": "yellow",
            },
            {"name": "name", "before": "foo", "after": "bar", "color": "yellow"},
            {
                "name": "proxiable",
                "before": "false",
                "after": "(known after apply)",
                "color": "yellow",
            },
            {"name": "tags", "before": "[]", "after": "null", "color": "red"},
            {
                "name": "ttl",
                "before": "1",
                "after": "(known after apply)",
                "color": "yellow",
            },
        ]
        for l in leaves:
            l1.add_leaf(
                f"[{l['color']}]{l['name']}[/{l['color']}] = {l['before']} -> {l['after']}"
            )
        l1.add_leaf("# (5 unchanged attributes hidden)")


class MockTfPlanViewerApp(App):
    BINDINGS = [("d", "toggle_dark", "Toggle dark mode")]

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()
        yield Footer()
        yield JsonTree("Plan Output")

    def action_toggle_dark(self) -> None:
        """An action to toggle dark mode."""
        self.theme = (
            "textual-dark" if self.theme == "textual-light" else "textual-light"
        )
