import click
import json
import mimetypes
import subprocess
import tempfile
from .tf_plan_viewer import TfPlanViewerApp

TF_BINARY = "tofu"

def create_tempfile(suffix = None):
    with tempfile.NamedTemporaryFile(delete=False) as temp_file:
        return temp_file.name

def load_json_file(file_path: str):
    with open(file_path, "r", encoding="utf-8") as file:
        return json.load(file)

def tf_plan(extra_args):
    temp_path = create_tempfile()
    plan_command = [TF_BINARY, "plan", f"-out={temp_path}"]
    plan_command.extend(extra_args)
    subprocess.run(plan_command, text=True, check=True)
    return temp_path

def tf_show(binary_plan_path):
    show_result = subprocess.run([TF_BINARY, "show", "-json", binary_plan_path], capture_output=True, text=True, check=True)
    return json.loads(show_result.stdout)

@click.command()
@click.option("--hide-experimental-warning", is_flag=True, default=False, help="Hide nasty warning.")
@click.option("--plan-file", required=False, type=click.Path(), help="Path to plan file (binary or JSON) to use.")
@click.argument("tf_args", nargs=-1, type=click.UNPROCESSED)
def run(hide_experimental_warning, plan_file, tf_args):
    if plan_file and mimetypes.guess_type(plan_file)[0] == "application/json":
        json_data = load_json_file(plan_file)
    elif plan_file:
        # Assume binary plan file
        json_data = tf_show(plan_file)
    else:
        binary_plan_path = tf_plan(tf_args)
        json_data = tf_show(binary_plan_path)
    app = TfPlanViewerApp(json_data, hide_experimental_warning)
    app.run()
