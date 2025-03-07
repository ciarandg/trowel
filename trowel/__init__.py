import click
from .tf_plan_viewer import TfPlanViewerApp

@click.command()
@click.option("--hide-experimental-warning", is_flag=True, default=False, help="Hide nasty warning.")
@click.argument("json_file_path")
def run(hide_experimental_warning, json_file_path):
    app = TfPlanViewerApp(json_file_path, hide_experimental_warning)
    app.run()
