import { serve, ServerRequest } from "./std/http/server.ts";

const addr = "127.0.0.1:4555";

async function proxyServer(): Promise<void> {
  const server = serve(addr);
  console.log(`Proxy server listening on http://${addr}/`);
  for await (const req of server) {
    proxyRequest(req);
  }
}

async function proxyRequest(req: ServerRequest): Promise<void> {
  console.log(`Proxy request to: ` + JSON.stringify(req.method));
  const resp = await fetch(req.url, {
    method: req.method,
    headers: req.headers,
  });
  req.respond({
    status: resp.status,
    body: new Uint8Array(await resp.arrayBuffer()),
    headers: resp.headers,
  });
}

proxyServer();

const client = Deno.createHttpClient({proxy: "http://localhost:4555"});
const res = await fetch("http://localhost:4545/std/examples/colors.ts", {client: client});

const body = new Uint8Array(await res.arrayBuffer());
await Deno.stdout.write(body);

