from http.server import BaseHTTPRequestHandler, HTTPServer


class RequestLoggerHandler(BaseHTTPRequestHandler):
    def _handle_request(self):
        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length).decode("utf-8", errors="replace")

        print("=== INCOMING REQUEST ===", flush=True)
        print(f"Method : {self.command}", flush=True)
        print(f"Path   : {self.path}", flush=True)
        print("Headers:", flush=True)
        for key, value in self.headers.items():
            print(f"  {key}: {value}", flush=True)
        if body:
            print(f"Body   :\n{body}", flush=True)
        print("========================\n", flush=True)

        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.end_headers()
        self.wfile.write(b"OK\n")

    def do_GET(self):
        self._handle_request()

    def do_POST(self):
        self._handle_request()

    def do_PUT(self):
        self._handle_request()

    def do_DELETE(self):
        self._handle_request()

    def do_PATCH(self):
        self._handle_request()

    def log_message(self, format, *args):
        # Suppress default access log format in favor of detailed output above
        pass


if __name__ == "__main__":
    server = HTTPServer(("127.0.0.1", 8080), RequestLoggerHandler)
    print("Python HTTP logger listening on 127.0.0.1:8080...", flush=True)
    server.serve_forever()
