const http = require("http");
const { URL } = require("url");

let users = [
  { id: 1, name: "Arun", email: "arun@test.com", role: "Admin", active: true },
  { id: 2, name: "Sachin", email: "sachin@test.com", role: "User", active: true },
  { id: 3, name: "Arun 2", email: "arun2@test.com", role: "User", active: false },
  { id: 4, name: "Sachin 2", email: "sachin2@test.com", role: "Manager", active: true }
];

let nextId = 5;

function sendJson(res, status, data) {
  res.writeHead(status, {
    "Content-Type": "application/json"
  });

  res.end(JSON.stringify(data, null, 2));
}

function sendError(res, status, message, details = null) {
  sendJson(res, status, {
    success: false,
    error: {
      message,
      details
    }
  });
}

function readJsonBody(req) {
  return new Promise((resolve, reject) => {
    let body = "";

    req.on("data", chunk => {
      body += chunk;

      // Prevent very large request bodies in mock server.
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
      } catch {
        reject(new Error("Invalid JSON body"));
      }
    });
  });
}

function validateUserInput(data, partial = false) {
  const errors = [];

  if (!partial || data.name !== undefined) {
    if (!data.name || typeof data.name !== "string" || data.name.trim().length < 2) {
      errors.push("Name must be at least 2 characters");
    }
  }

  if (!partial || data.email !== undefined) {
    if (
      !data.email ||
      typeof data.email !== "string" ||
      !data.email.includes("@")
    ) {
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

function getUserIdFromPath(pathname) {
  const match = pathname.match(/^\/api\/users\/(\d+)$/);
  return match ? Number(match[1]) : null;
}

function listUsers(req, res, url) {
  const search = (url.searchParams.get("search") || "").toLowerCase();
  const role = url.searchParams.get("role");
  const active = url.searchParams.get("active");
  const page = Math.max(Number(url.searchParams.get("page") || 1), 1);
  const limit = Math.min(Math.max(Number(url.searchParams.get("limit") || 10), 1), 100);

  let result = [...users];

  if (search) {
    result = result.filter(user => {
      return (
        user.name.toLowerCase().includes(search) ||
        user.email.toLowerCase().includes(search)
      );
    });
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

  const total = result.length;
  const start = (page - 1) * limit;
  const pagedUsers = result.slice(start, start + limit);

  sendJson(res, 200, pagedUsers);
}

function getStats(req, res) {
  sendJson(res, 200, {
    totalUsers: users.length,
    activeUsers: users.filter(user => user.active).length,
    inactiveUsers: users.filter(user => !user.active).length,
    roles: users.reduce((acc, user) => {
      acc[user.role] = (acc[user.role] || 0) + 1;
      return acc;
    }, {})
  });
}

const server = http.createServer(async (req, res) => {
  try {
    if (applyCors(req, res)) {
      return;
    }

    const url = new URL(req.url, "http://localhost:4000");
    const pathname = url.pathname;

    // Health check
    if (pathname === "/api/health" && req.method === "GET") {
      sendJson(res, 200, {
        success: true,
        message: "Mock API is running"
      });
      return;
    }

    // Stats
    if (pathname === "/api/stats" && req.method === "GET") {
      getStats(req, res);
      return;
    }

    // List users
    if (pathname === "/api/users" && req.method === "GET") {
      listUsers(req, res, url);
      return;
    }

    // Create user
    if (pathname === "/api/users" && req.method === "POST") {
      const data = await readJsonBody(req);
      const errors = validateUserInput(data);

      if (errors.length > 0) {
        sendError(res, 400, "Validation failed", errors);
        return;
      }

      const duplicate = users.some(
        user => user.email.toLowerCase() === data.email.toLowerCase()
      );

      if (duplicate) {
        sendError(res, 409, "Email already exists");
        return;
      }

      const user = {
        id: nextId++,
        name: data.name.trim(),
        email: data.email.trim(),
        role: data.role || "User",
        active: data.active ?? true
      };

      users.push(user);

      sendJson(res, 201, user);

      return;
    }

    const userId = getUserIdFromPath(pathname);

    // Get single user
    if (userId && req.method === "GET") {
      const user = users.find(item => item.id === userId);

      if (!user) {
        sendError(res, 404, "User not found");
        return;
      }

      sendJson(res, 200, user);

      return;
    }

    // Replace user
    if (userId && req.method === "PUT") {
      const data = await readJsonBody(req);
      const errors = validateUserInput(data);

      if (errors.length > 0) {
        sendError(res, 400, "Validation failed", errors);
        return;
      }

      const index = users.findIndex(item => item.id === userId);

      if (index === -1) {
        sendError(res, 404, "User not found");
        return;
      }

      users[index] = {
        id: userId,
        name: data.name.trim(),
        email: data.email.trim(),
        role: data.role || "User",
        active: data.active ?? true
      };

      sendJson(res, 200, users[index]);

      return;
    }

    // Partial update user
    if (userId && req.method === "PATCH") {
      const data = await readJsonBody(req);
      const errors = validateUserInput(data, true);

      if (errors.length > 0) {
        sendError(res, 400, "Validation failed", errors);
        return;
      }

      const user = users.find(item => item.id === userId);

      if (!user) {
        sendError(res, 404, "User not found");
        return;
      }

      if (data.name !== undefined) user.name = data.name.trim();
      if (data.email !== undefined) user.email = data.email.trim();
      if (data.role !== undefined) user.role = data.role;
      if (data.active !== undefined) user.active = data.active;

      sendJson(res, 200, user);

      return;
    }

    // Delete user
    if (userId && req.method === "DELETE") {
      const before = users.length;
      users = users.filter(item => item.id !== userId);

      if (users.length === before) {
        sendError(res, 404, "User not found");
        return;
      }

      sendJson(res, 200, {
        id: userId,
        deleted: true
      });

      return;
    }

    sendError(res, 404, "Route not found");
  } catch (error) {
    sendError(res, 500, error.message || "Internal server error");
  }
});

server.listen(4000, () => {
  console.log("Mock API running: http://localhost:4000");
  console.log("Health: http://localhost:4000/api/health");
  console.log("Users:  http://localhost:4000/api/users");
});