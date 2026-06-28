#!/usr/bin/env node

const readline = require("readline");
const spawn = require("cross-spawn");
const path = require("path");
const fs = require("fs");

function askProjectName(defaultValue = "my-korlix-app") {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  return new Promise((resolve) => {
    rl.question(`Project name (${defaultValue}): `, (answer) => {
      rl.close();
      resolve(answer.trim() || defaultValue);
    });
  });
}

function printUsage() {
  console.error("");
  console.error("Usage:");
  console.error("  npm create korlix@latest");
  console.error("  npm create korlix@latest my-app");
  console.error("  npm create korlix@latest my-app -- --install");
  console.error("  npx --prefer-online create-korlix@latest my-app");
  console.error("");
}

function parseArgs(argv) {
  const options = {
    install: false,
    start: false,
    projectName: undefined
  };

  for (const arg of argv) {
    if (arg === "--help" || arg === "-h") {
      printUsage();
      process.exit(0);
    }

    if (arg === "--install" || arg === "--immediate") {
      options.install = true;
      continue;
    }

    if (arg === "--start") {
      options.install = true;
      options.start = true;
      continue;
    }

    if (arg.startsWith("-")) {
      console.error("");
      console.error(`Error: unknown option: ${arg}`);
      printUsage();
      process.exit(1);
    }

    if (options.projectName) {
      console.error("");
      console.error(`Error: unexpected argument: ${arg}`);
      printUsage();
      process.exit(1);
    }

    options.projectName = arg;
  }

  return options;
}

function validateProjectName(projectName) {
  if (!projectName) {
    return;
  }

  if (projectName.startsWith("-")) {
    console.error("");
    console.error(`Error: invalid project name: ${projectName}`);
    console.error("");
    console.error("This looks like an npm/npx option, not a Korlix project name.");
    console.error("");
    console.error("Correct command:");
    console.error("  npx --prefer-online create-korlix@latest my-app");
    console.error("");
    console.error("Wrong command:");
    console.error("  npx create-korlix@latest --prefer-online");
    printUsage();
    process.exit(1);
  }
}

function run(command, args, cwd = process.cwd()) {
  const result = spawn.sync(command, args, {
    cwd,
    stdio: "inherit",
    shell: false,
    windowsHide: true,
    env: process.env
  });

  if (result.error) {
    console.error(result.error);
    process.exit(1);
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function detectPackageManager() {
  const userAgent = process.env.npm_config_user_agent || "";
  const name = userAgent.split(" ")[0].split("/")[0];

  if (name === "yarn") {
    return {
      name,
      install: ["yarn"],
      dev: ["yarn", "dev"]
    };
  }

  if (name === "pnpm") {
    return {
      name,
      install: ["pnpm", "install"],
      dev: ["pnpm", "dev"]
    };
  }

  if (name === "bun") {
    return {
      name,
      install: ["bun", "install"],
      dev: ["bun", "run", "dev"]
    };
  }

  return {
    name: "npm",
    install: ["npm", "install"],
    dev: ["npm", "run", "dev"]
  };
}

function formatCommand(command) {
  return command.join(" ");
}

function formatCdTarget(projectName) {
  return /\s/.test(projectName) ? `"${projectName}"` : projectName;
}

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2) + "\n");
}

function findKorlixCliScript() {
  return require.resolve("korlix/bin/korlix.js", {
    paths: [__dirname, process.cwd()]
  });
}

async function main() {
  console.log("");
  console.log("Korlix - frontend-first language");
  console.log("");

  const options = parseArgs(process.argv.slice(2));
  let projectName = options.projectName;

  validateProjectName(projectName);

  if (!projectName) {
    projectName = await askProjectName();
    validateProjectName(projectName);
  }

  const projectPath = path.resolve(process.cwd(), projectName);

  if (fs.existsSync(projectPath)) {
    console.error("");
    console.error(`Error: folder already exists: ${projectName}`);
    process.exit(1);
  }

  console.log(`Creating Korlix app: ${projectName}`);
  console.log("");

  const korlixCliScript = findKorlixCliScript();

  run(process.execPath, [korlixCliScript, "new", projectName], process.cwd());

  const configPath = path.join(projectPath, "korlix.config.json");

  if (fs.existsSync(configPath)) {
    const config = JSON.parse(fs.readFileSync(configPath, "utf8"));
    config.mode = "spa";
    writeJson(configPath, config);
  }

  const packagePath = path.join(projectPath, "package.json");

  if (fs.existsSync(packagePath)) {
    const pkg = JSON.parse(fs.readFileSync(packagePath, "utf8"));

    pkg.scripts = {
      dev: "korlix dev",
      build: "korlix build",
      preview: "korlix preview",
      check: "korlix check"
    };

    pkg.devDependencies = {
      ...(pkg.devDependencies || {}),
      korlix: "0.1.2"
    };

    writeJson(packagePath, pkg);
  }

  const packageManager = detectPackageManager();
  const cdTarget = formatCdTarget(projectName);

  console.log("");
  console.log("Korlix app created");
  console.log("");
  if (options.install) {
    console.log("Installing dependencies...");
    console.log("");
    run(packageManager.install[0], packageManager.install.slice(1), projectPath);

    if (options.start) {
      console.log("");
      console.log("Starting dev server...");
      console.log("");
      run(packageManager.dev[0], packageManager.dev.slice(1), projectPath);
    } else {
      console.log("");
      console.log("Next step:");
      console.log(`  cd ${cdTarget}`);
      console.log(`  ${formatCommand(packageManager.dev)}`);
    }
  } else {
    console.log("Next steps:");
    console.log(`  cd ${cdTarget}`);
    console.log(`  ${formatCommand(packageManager.install)}`);
    console.log(`  ${formatCommand(packageManager.dev)}`);
    console.log("");
    console.log("To install during creation, run:");
    console.log(`  npm create korlix@latest ${projectName} -- --install`);
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
