Yes. Test it in **3 layers**: compiler test, generated JS test, browser frontend test.

Important: your compiler embeds this file directly:

```txt id="1dynm0"
crates/korlix-compiler/runtime-bundle/korlix.runtime.js
```

So if you only updated `runtime/src/api/*`, the browser may not see it yet. Your master plan says the runtime bundle is embedded using `include_str!("../runtime-bundle/korlix.runtime.js")`, so frontend testing needs that embedded runtime updated too. 

## 1. First run compiler tests

From Korlix root:

```bash id="g846u0"
cargo test
```

Then test only parser/lexer if needed:

```bash id="ywfhfg"
cargo test -p korlix-lexer
cargo test -p korlix-parser
cargo test -p korlix-codegen
cargo test -p korlix-compiler
```

This confirms:

```txt id="xlz5va"
get users "/api/users"
post "/api/users" {...}
reload users
```

are lexed, parsed, and compiled.

---

## 2. Create a mock API server

Create this file outside your Korlix test app:

```txt id="o0q4na"
mock-api.js
```

```js id="ovzpce"
const http = require("http");

let users = [
  { id: 1, name: "Arun", email: "arun@test.com" },
  { id: 2, name: "Sachin", email: "sachin@test.com" }
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
```

Run it:

```bash id="pjr5hk"
node mock-api.js
```

---

## 3. Create a frontend test page

In your Korlix app, create or update:

```txt id="vs5e9m"
src/pages/api-test.klx
```

Use full URL first, because your Korlix dev server serves frontend files and the API is on another port.

```klx id="hfotuk"
page ApiTest route "/api-test":
  get users "http://localhost:4000/api/users"

  state name = ""
  state email = ""

  action save:
    post "http://localhost:4000/api/users" { name: name, email: email }
    reload users

  section .p-6
    h1 .text-2xl .font-bold "API Test"

    if usersLoading
      p .text-muted "Loading users..."

    if usersError
      p .text-danger "{usersError}"

    div .flex .gap-2 .mb-4
      input .border .p-2 value=name placeholder="Name"
      input .border .p-2 value=email placeholder="Email"
      button .bg-blue-600 .text-white .px-4 .py-2 on:click=save "Save"

    for user in users:
      div .border .rounded-lg .p-3 .mb-2
        h2 .font-semibold "{user.name}"
        p .text-muted "{user.email}"
```

Then make sure this route is included in your app route config if your current app requires route registration.

---

## 4. Run the Korlix frontend

From your Korlix app folder:

```bash id="ylr3ra"
korlix dev
```

Or from source:

```bash id="k01qyi"
cargo run -p korlix-cli -- dev
```

Open:

```txt id="8sgjtb"
http://localhost:3000/api-test
```

Your dev server already performs initial build, serves `dist/`, watches `.klx` changes, and sends HMR messages. 

---

## 5. What to check in browser

Open DevTools → Network.

You should see:

```txt id="u88jtx"
GET http://localhost:4000/api/users       200
POST http://localhost:4000/api/users      201
GET http://localhost:4000/api/users       200 after reload users
```

Open Console and check:

```js id="8bnuzw"
window.KorlixRuntime.api
```

Expected:

```txt id="ahz20u"
query
request
reload
```

If this is `undefined`, your runtime bundle is not updated.

---

## 6. Check generated files

After running `korlix build` or `korlix dev`, inspect:

```txt id="o7e0o6"
dist/app.js
dist/korlix.runtime.js
```

Search for:

```txt id="10bfnn"
KorlixRuntime.api.query
KorlixRuntime.api.request
KorlixRuntime.api.reload
```

Expected generated JS:

```js id="khy8xv"
KorlixRuntime.api.query("users", "http://localhost:4000/api/users");
```

For action:

```js id="pyiazt"
await KorlixRuntime.api.request("POST", "http://localhost:4000/api/users", {
  name: name,
  email: email
});

await KorlixRuntime.api.reload("users");
```

## Most common issue

If compile works but frontend does nothing, the problem is usually this:

```txt id="rzx5aw"
You updated runtime/src/api, but the compiler still embeds crates/korlix-compiler/runtime-bundle/korlix.runtime.js
```

So update the embedded runtime bundle or wire your TypeScript runtime build into that file before testing in the browser.
