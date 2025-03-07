import sys
from .tf_plan_viewer import TfPlanViewerApp


def run():
    json_file_path = sys.argv[1]
    app = TfPlanViewerApp(json_file_path)
    app.run()
