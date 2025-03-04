import sys
from .mock_tf_plan_viewer import MockTfPlanViewerApp
from .tf_plan_viewer import TfPlanViewerApp

def run():
    json_file_path = sys.argv[1]
    # app = MockTfPlanViewerApp()
    app = TfPlanViewerApp(json_file_path)
    app.run()
