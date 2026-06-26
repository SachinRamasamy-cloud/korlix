# create-korlix

Create a new Korlix app.

Usage:

npm create korlix@latest my-app

Or without a folder name:

npm create korlix@latest

If no folder name is provided, Korlix asks for the project name.

Korlix apps are created in SPA mode by default.

Publish:

```sh
cd npm/create-korlix
npm login
npm run publish:dry
npm run publish:public
```

If your npm account has two-factor authentication enabled, pass the current
6-digit authenticator code:

```sh
npm run publish:public -- --otp=123456
```

Publish `korlix` first, then publish `create-korlix`.
