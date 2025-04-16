import http.server
import urllib.parse

experiments = {}

class ExperimentHTTPRequestHandler(http.server.BaseHTTPRequestHandler):
	def do_POST(self):
		content_length = int(self.headers.get('Content-Length', 0))
		post_data = self.rfile.read(content_length).decode('utf-8')
		parsed_data = urllib.parse.parse_qs(post_data)
		
		experiment_id = parsed_data.get('experiment_id', [None])[0]
		status = parsed_data.get('status', [None])[0]
		total = parsed_data.get('total', [None])[0]

		if experiment_id is None or status is None or total is None:
			self.send_response(400)
			self.send_header('Content-type', 'text/plain')
			self.end_headers()
			self.wfile.write(b"Missing one or more parameters: experiment_id, status, total")
			return

		try:
			status = int(status)
			total = int(total)
		except ValueError:
			self.send_response(400)
			self.send_header('Content-type', 'text/plain')
			self.end_headers()
			self.wfile.write(b"Status and total must be integers.")
			return

		experiments[experiment_id] = (status, total)

		self.send_response(200)
		self.send_header('Content-type', 'text/plain')
		self.end_headers()
		self.wfile.write(b"Experiment updated successfully.")

	def do_GET(self):
		self.send_response(200)
		self.send_header('Content-type', 'text/html')
		self.end_headers()

		response_html = """
		<html>
		<head>
			<title>Experiment Progress</title>
			<meta name="viewport" content="width=device-width, initial-scale=1.0" />
			<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">
		</head>
		<body>
			<h1>Experiment Progress</h1>
			<table border="1" cellspacing="0" cellpadding="5">
				<tr>
					<th>Experiment ID</th>
					<th>Status</th>
					<th>Total</th>
					<th>Percent Complete</th>
				</tr>
		"""
		for exp_id, (status, total) in experiments.items():
			percent = (status / total * 100) if total else 0
			response_html += f"""
				<tr>
					<td>{exp_id}</td>
					<td>{status}</td>
					<td>{total}</td>
					<td>{percent:.2f}%</td>
				</tr>
			"""
		response_html += """
			</table>
		</body>
		</html>
		"""
		self.wfile.write(response_html.encode('utf-8'))

def run(server_class=http.server.HTTPServer,handler_class=ExperimentHTTPRequestHandler,port=25565):
	server_address = ('', port)
	httpd = server_class(server_address, handler_class)
	print(f"Server running on port {port}...")
	httpd.serve_forever()

if __name__ == '__main__':
	run()
