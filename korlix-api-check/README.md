# Korlix API Check

## Start the API

```bash
node api-server.js
```

Test:

```bash
curl http://localhost:8787/api/health
```

## Start Korlix

```bash
korlix check
korlix dev
```

Open the URL printed by Korlix and visit `/api-check`.

Optional `package.json` script:

```json
{
  "scripts": {
    "api": "node api-server.js"
  }
}
```
