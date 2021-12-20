# Python 3 server example
import http.server
import time
import socketserver

hostName = "0.0.0.0"
serverPort = 80

DIRECTORY="/usr/local/bin/wasm"

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

class MyServer(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/':
            self.path = 'wasm/index.html'
        ## self.send_response(200)
        ## self.send_header("Content-type", "text/html")
        ## self.end_headers()
        return(http.server.SimpleHTTPRequestHandler.do_GET(self.path))

if __name__ == "__main__":        
   webServer = http.server.HTTPServer((hostName, serverPort), Handler)
    #print("serving at port", PORT)
        #webServer = http.server.HTTPServer((hostName, serverPort), Handler)
   print("Server started http://%s:%s" % (hostName, serverPort))

   try:
       webServer.serve_forever()
   except KeyboardInterrupt:
       pass

   webServer.server_close()
   print("Server stopped.")
