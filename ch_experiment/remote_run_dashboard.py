import http.server
import socketserver
import threading
import subprocess
import sys
import time

output_lines = []
start_time = None
end_time = None
command_running = True
output_lock = threading.Lock()

def run_command(cmd):
    global start_time, end_time, command_running
    start_time = time.time()
    
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1)
    
    for line in process.stdout:
        with output_lock:
            output_lines.append(line)
            if len(output_lines) > 1000:
                output_lines.pop(0)
        print(line, end="")
    
    process.stdout.close()
    process.wait()
    end_time = time.time()
    command_running = False

class MonitorHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path in ("/", "/index.html"):
            with output_lock:
                last_10 = output_lines[-10:]
            output_html = "<pre>" + "".join(last_10) + "</pre>"

            if start_time is None:
                elapsed = 0
            elif end_time is not None:
                elapsed = int(end_time - start_time)
            else:
                elapsed = int(time.time() - start_time)

            finished_message = ""
            if not command_running and end_time is not None:
                finished_message = "<h1 style='font-size:3em;'>Finished</h1>"

            html_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Command Output Monitor</title>
    <script type="text/javascript">
        setTimeout(function() {{
            window.location.reload();
        }}, 15000);
    </script>
</head>
<body>
    <h2>Elapsed Time: {elapsed} seconds</h2>
    {output_html}
    {finished_message}
</body>
</html>
"""
            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.end_headers()
            self.wfile.write(html_content.encode("utf-8"))
        else:
            self.send_error(404, "File not found.")

def run_web_server(port=8080):
    server_address = ("", port)
    httpd = http.server.HTTPServer(server_address, MonitorHandler)
    print(f"Serving at http://localhost:{port}")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("Shutting down server...")
    httpd.server_close()

if __name__ == '__main__':
    if len(sys.argv) < 2:
        sys.exit(1)
    command_to_run = sys.argv[1:]
    print(f"Executing command: {' '.join(command_to_run)}")
    command_thread = threading.Thread(target=run_command, args=(command_to_run,))
    command_thread.daemon = True
    command_thread.start()
    run_web_server(port=25565)
