const http = require("node:http");

const HOST = process.env.API_HOST || "127.0.0.1";
const PORT = Number.parseInt(process.env.API_PORT || "8787", 10);

function setCorsHeaders(response) {
  response.setHeader("Access-Control-Allow-Origin", "*");
  response.setHeader("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
  response.setHeader(
    "Access-Control-Allow-Headers",
    "Content-Type, Authorization"
  );
}

function sendJson(response, statusCode, payload) {
  setCorsHeaders(response);
  response.writeHead(statusCode, {
    "Content-Type": "application/json; charset=utf-8",
    "Cache-Control": "no-store"
  });
  response.end(JSON.stringify(payload, null, 2));
}

async function readJsonBody(request) {
  const chunks = [];

  for await (const chunk of request) {
    chunks.push(chunk);
  }

  if (chunks.length === 0) {
    return {};
  }

  try {
    return JSON.parse(Buffer.concat(chunks).toString("utf8"));
  } catch {
    const error = new Error("Request body must contain valid JSON.");
    error.statusCode = 400;
    throw error;
  }
}

const server = http.createServer(async (request, response) => {
  const method = request.method || "GET";
  const url = new URL(
    request.url || "/",
    `http://${request.headers.host || `${HOST}:${PORT}`}`
  );

  if (method === "OPTIONS") {
    setCorsHeaders(response);
    response.writeHead(204);
    response.end();
    return;
  }

  if (method === "GET" && url.pathname === "/") {
    sendJson(response, 200, {
      message: "Korlix API test server is running.",
      endpoints: {
        health: "GET /api/health",
        echo: "POST /api/echo"
      }
    });
    return;
  }

  if (method === "GET" && url.pathname === "/api/health") {
    sendJson(response, 200, {
      status: "ok",
      service: "korlix-api-test",
      message: "The frontend successfully reached the API.",
      timestamp: new Date().toISOString()
    });
    return;
  }

  if (method === "POST" && url.pathname === "/api/echo") {
    try {
      const body = await readJsonBody(request);

      sendJson(response, 200, {
        status: "ok",
        message: "The API received the request body.",
        received: body,
        timestamp: new Date().toISOString()
      });
    } catch (error) {
      sendJson(response, error.statusCode || 500, {
        status: "error",
        message: error.message || "Unexpected server error."
      });
    }
    return;
  }

  sendJson(response, 404, {
    status: "error",
    message: "Route not found",
    method,
    path: url.pathname
  });
});

server.listen(PORT, HOST, () => {
  console.log(`Korlix API test server: http://${HOST}:${PORT}`);
  console.log(`Health endpoint:       http://${HOST}:${PORT}/api/health`);
});

function shutdown(signal) {
  console.log(`\nReceived ${signal}. Stopping API server...`);
  server.close((error) => {
    if (error) {
      console.error("Failed to stop API server cleanly:", error);
      process.exitCode = 1;
      return;
    }

    process.exit(0);
  });
}

process.on("SIGINT", () => shutdown("SIGINT"));
process.on("SIGTERM", () => shutdown("SIGTERM"));
