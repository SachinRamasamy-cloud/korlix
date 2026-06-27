const http = require("http");

let users = [
  { id: 1, name: "Arun", email: "arun@test.com" },
  { id: 2, name: "Sachin", email: "sachin@test.com" },
  { id: 3, name: "Arun 2", email: "arun@test.com" },
  { id: 4, name: "Sachin 2", email: "sachin@test.com" }
];

const server = http.createServer((req, res) => {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET,POST,PUT,PATCH,DELETE,OPTIONS");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type");

  if (req.method === "OPTIONS") {
    res.writeHead(204);
    res.end();
    return;
  }

  if (req.url === "/api/users" && req.method === "GET") {
    res.writeHead(200, { "Content-Type": "application/json" });
    res.end(JSON.stringify(users));
    return;
  }

  if (req.url === "/api/users" && req.method === "POST") {
    let body = "";

    req.on("data", chunk => {
      body += chunk;
    });

    req.on("end", () => {
      const data = JSON.parse(body || "{}");

      const user = {
        id: Date.now(),
        name: data.name || "Unnamed",
        email: data.email || "no-email@test.com"
      };

      users.push(user);

      res.writeHead(201, { "Content-Type": "application/json" });
      res.end(JSON.stringify(user));
    });

    return;
  }

  res.writeHead(404, { "Content-Type": "application/json" });
  res.end(JSON.stringify({ error: "Not found" }));
});

server.listen(4000, () => {
  console.log("Mock API running: http://localhost:4000");
});