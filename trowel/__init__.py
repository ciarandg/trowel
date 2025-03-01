import sys
from .json_viewer import JsonViewerApp

def run():
    json_file_path = sys.argv[1]
    app = JsonViewerApp(json_file_path)
    app.run()
