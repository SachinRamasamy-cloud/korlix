import http from "node:http";

let users = [
  { id: 1, name: "Aarav Sharma", role: "Compiler Engineer", active: true },
  { id: 2, name: "Maya Patel", role: "Frontend Developer", active: true },
  { id: 3, name: "Noah Williams", role: "Documentation Author", active: false }
];
let nextId = 4;

const json = (response, status, body) => {
  response.writeHead(status, {
    "Content-Type": "application/json; charset=utf-8",
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Headers": "Content-Type, Authorization",
    "Access-Control-Allow-Methods": "GET, POST, PUT, PATCH, DELETE, OPTIONS"
  });
  response.end(JSON.stringify(body));
};

const readBody = async request => {
  const chunks = [];
  for await (const chunk of request) chunks.push(chunk);
  if (chunks.length === 0) return {};
  try {
    return JSON.parse(Buffer.concat(chunks).toString("utf8"));
  } catch {
    return {};
  }
};

const server = http.createServer(async (request, response) => {
  const url = new URL(request.url ?? "/", "http://localhost:8787");

  if (request.method === "OPTIONS") {
    json(response, 204, null);
    return;
  }

  if (url.pathname === "/api/health") {
    json(response, 200, { status: "ok", service: "korlix-showcase-api" });
    return;
  }

  if (url.pathname === "/api/users" && request.method === "GET") {
    json(response, 200, users);
    return;
  }

  if (url.pathname === "/api/users" && request.method === "POST") {
    const body = await readBody(request);
    const user = {
      id: nextId++,
      name: body.name ?? "New Korlix User",
      role: body.role ?? "Developer",
      active: body.active ?? true
    };
    users.push(user);
    json(response, 201, user);
    return;
  }

  const match = url.pathname.match(/^\/api\/users\/(\d+)$/);
  if (match) {
    const id = Number(match[1]);
    const index = users.findIndex(user => user.id === id);
    if (index < 0) {
      json(response, 404, { message: "User not found" });
      return;
    }

    if (request.method === "PUT") {
      const body = await readBody(request);
      users[index] = {
        id,
        name: body.name ?? users[index].name,
        role: body.role ?? users[index].role,
        active: body.active ?? users[index].active
      };
      json(response, 200, users[index]);
      return;
    }

    if (request.method === "PATCH") {
      const body = await readBody(request);
      users[index] = { ...users[index], ...body, id };
      json(response, 200, users[index]);
      return;
    }

    if (request.method === "DELETE") {
      const [removed] = users.splice(index, 1);
      json(response, 200, removed);
      return;
    }
  }

  json(response, 404, { message: "Route not found" });
});

server.listen(8787, "127.0.0.1", () => {
  console.log("Korlix showcase API running at http://localhost:8787");
});
