import click
from .tf_plan_viewer import TfPlanViewerApp

@click.command()
@click.argument("json_file_path")
def run(json_file_path):
    app = TfPlanViewerApp(json_file_path)
    app.run()
