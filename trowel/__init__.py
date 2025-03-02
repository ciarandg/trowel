import sys
from .mock_tf_plan_viewer import MockTfPlanViewerApp

def run():
    # json_file_path = sys.argv[1]
    app = MockTfPlanViewerApp()
    app.run()
