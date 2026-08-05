const http = require("http");
const { URL } = require("url");

let users = [
  { id: 1, name: "Arun", email: "arun@test.com", role: "Admin", active: true },
  { id: 2, name: "Sachin", email: "sachin@test.com", role: "User", active: true },
  { id: 3, name: "Priya", email: "priya@test.com", role: "User", active: false }
];
let nextId = 4;

function sendJson(res, status, data) {
  res.writeHead(status, {
    "Content-Type": "application/json"
  });
  res.end(JSON.stringify(data, null, 2));
}

function sendError(res, status, message, details = null) {
  sendJson(res, status, {
    success: false,
    error: { message, details }
  });
}

function readJsonBody(req) {
  return new Promise((resolve, reject) => {
    let body = "";

    req.on("data", chunk => {
      body += chunk;
      if (body.length > 1_000_000) {
        reject(new Error("Request body too large"));
        req.destroy();
      }
    });

    req.on("end", () => {
      if (!body.trim()) {
        resolve({});
        return;
      }
      try {
        resolve(JSON.parse(body));
      } catch (err) {
        reject(new Error("Invalid JSON body"));
      }
    });
  });
}

function applyCors(req, res) {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET,POST,PUT,PATCH,DELETE,OPTIONS");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization");

  if (req.method === "OPTIONS") {
    res.writeHead(204);
    res.end();
    return true;
  }

  return false;
}

function validateUser(data, partial = false) {
  const errors = [];

  if (!partial || data.name !== undefined) {
    if (!data.name || typeof data.name !== "string" || data.name.trim().length < 2) {
      errors.push("Name must be at least 2 characters");
    }
  }

  if (!partial || data.email !== undefined) {
    if (!data.email || typeof data.email !== "string" || !data.email.includes("@")) {
      errors.push("Email must be valid");
    }
  }

  if (data.role !== undefined && typeof data.role !== "string") {
    errors.push("Role must be a string");
  }

  if (data.active !== undefined && typeof data.active !== "boolean") {
    errors.push("Active must be true or false");
  }

  return errors;
}

function getUserId(pathname) {
  const match = pathname.match(/^\/api\/users\/(\d+)$/);
  return match ? Number(match[1]) : null;
}

function listUsers(url, res) {
  const search = (url.searchParams.get("search") || "").toLowerCase();
  const role = url.searchParams.get("role");
  const active = url.searchParams.get("active");

  let result = [...users];

  if (search) {
    result = result.filter(user =>
      user.name.toLowerCase().includes(search) ||
      user.email.toLowerCase().includes(search)
    );
  }

  if (role) {
    result = result.filter(user => user.role.toLowerCase() === role.toLowerCase());
  }

  if (active === "true") {
    result = result.filter(user => user.active === true);
  }
  if (active === "false") {
    result = result.filter(user => user.active === false);
  }

  sendJson(res, 200, result);
}

function sendStats(res) {
  sendJson(res, 200, {
    totalUsers: users.length,
    activeUsers: users.filter(u => u.active).length,
    inactiveUsers: users.filter(u => !u.active).length,
    roles: users.reduce((acc, user) => {
      acc[user.role] = (acc[user.role] || 0) + 1;
      return acc;
    }, {})
  });
}

const server = http.createServer(async (req, res) => {
  try {
    if (applyCors(req, res)) return;

    const url = new URL(req.url, "http://localhost:4001");
    const pathname = url.pathname;

    if (pathname === "/api/health" && req.method === "GET") {
      sendJson(res, 200, { success: true, message: "Demo API is running" });
      return;
    }

    if (pathname === "/api/stats" && req.method === "GET") {
      sendStats(res);
      return;
    }

    if (pathname === "/api/users" && req.method === "GET") {
      listUsers(url, res);
      return;
    }

    if (pathname === "/api/users" && req.method === "POST") {
      const payload = await readJsonBody(req);
      const errors = validateUser(payload);
      if (errors.length) {
        sendError(res, 400, "Validation failed", errors);
        return;
      }

      if (users.some(u => u.email.toLowerCase() === payload.email.toLowerCase())) {
        sendError(res, 409, "Email already exists");
        return;
      }

      const user = {
        id: nextId++,
        name: payload.name.trim(),
        email: payload.email.trim(),
        role: payload.role || "User",
        active: payload.active ?? true
      };

      users.push(user);
      sendJson(res, 201, user);
      return;
    }

    const userId = getUserId(pathname);
    if (userId && req.method === "GET") {
      const user = users.find(u => u.id === userId);
      if (!user) {
        sendError(res, 404, "User not found");
        return;
      }
      sendJson(res, 200, user);
      return;
    }

    if (userId && req.method === "PATCH") {
      const payload = await readJsonBody(req);
      const errors = validateUser(payload, true);
      if (errors.length) {
        sendError(res, 400, "Validation failed", errors);
        return;
      }

      const user = users.find(u => u.id === userId);
      if (!user) {
        sendError(res, 404, "User not found");
        return;
      }

      if (payload.name !== undefined) user.name = payload.name.trim();
      if (payload.email !== undefined) user.email = payload.email.trim();
      if (payload.role !== undefined) user.role = payload.role;
      if (payload.active !== undefined) user.active = payload.active;

      sendJson(res, 200, user);
      return;
    }

    if (userId && req.method === "DELETE") {
      const before = users.length;
      users = users.filter(u => u.id !== userId);
      if (users.length === before) {
        sendError(res, 404, "User not found");
        return;
      }
      sendJson(res, 200, { deleted: true, id: userId });
      return;
    }

    sendError(res, 404, "Route not found");
  } catch (err) {
    sendError(res, 500, err.message || "Internal server error");
  }
});

server.listen(4001, () => {
  console.log("Demo API running: http://localhost:4001");
  console.log("Health: http://localhost:4001/api/health");
  console.log("Users:  http://localhost:4001/api/users");
  console.log("Stats:  http://localhost:4001/api/stats");
});
