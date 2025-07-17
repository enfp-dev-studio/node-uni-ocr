


# @enfpdev/node-uni-ocr

![CI](https://github.com/enfp-dev-studio/node-uni-ocr/actions/workflows/CI.yml/badge.svg)

## Description

**@enfpdev/node-uni-ocr** is a native Node.js library that brings system-level OCR (Optical Character Recognition) to Node.js and Electron by porting the [uniOCR](https://github.com/hiroi-sora/uni-ocr) engine (written in Rust) via [napi-rs](https://napi.rs/).

It enables easy, cross-platform OCR functionality (Windows, macOS, and experimental WASI/Linux) without requiring users to set up complex build tools or environments.

---


## Installation

```bash
pnpm add @enfpdev/node-uni-ocr
# or
npm install @enfpdev/node-uni-ocr
```

## Usage

```js
const { recognize } = require('@enfpdev/node-uni-ocr');

// OCR from file path
const result = await recognize('test.png');
console.log(result.text, result.confidence);

// OCR from buffer
const fs = require('fs');
const buffer = fs.readFileSync('test.png');
const result2 = await recognize(buffer, {
  languages: ['en', 'ko'],
  confidence_threshold: 0.5,
  timeout: 10,
});
console.log(result2.text);
```

### Usage in Electron

You can use this library in Electron just like in Node.js (recommended: main process).

---

pnpm add @napi-rs/package-template
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]
```bash
## Requirements

- Latest Rust toolchain
- Node.js 18 or higher (Node-API supported)
- corepack enabled (for pnpm)

## Build & Test

```bash
pnpm install
pnpm build
pnpm test
```

## Release

Make sure your **NPM_TOKEN** is set in GitHub Secrets. To release a new version:

```bash
npm version [patch|minor|major]

```

GitHub Actions will handle the rest (build, test, and publish).

---

## License

MIT
MIT
```
