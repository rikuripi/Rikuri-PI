from http.server import BaseHTTPRequestHandler,ThreadingHTTPServer
import socketserver,threading,json
class H(BaseHTTPRequestHandler):
 def do_GET(self):
  print(json.dumps({"protocol":"tcp","path":self.path}),flush=True)
  self.send_response(200); self.end_headers(); self.wfile.write(b"synthetic-network-ok")
 def log_message(self,*a): pass
class U(socketserver.BaseRequestHandler):
 def handle(self):
  d,s=self.request; print(json.dumps({"protocol":"udp","marker":d.decode()}),flush=True); s.sendto(d,self.client_address)
udp=socketserver.ThreadingUDPServer(("0.0.0.0",8088),U)
threading.Thread(target=udp.serve_forever,daemon=True).start()
ThreadingHTTPServer(("0.0.0.0",8088),H).serve_forever()
