# WASM in NextJS Demo Project

This project demos a WASM module with functions for creating a `greeting` and to `sum` numbers.

The project demos two patterns for using WebAssembly modules `clientside` with Next;

- Using a Context to share the WASM library to any children
- Importing and initializing (`wasm.default()`) the library inside the same component

The WASM module could be built to target `nodejs` as well.

## Tooling

This project uses [Wasm-Pack](https://github.com/rustwasm/wasm-pack) to build rust-generated WebAssembly packages with JavaScript interop.
Wasm-Pack is part of the [rust-wasm](https://github.com/rustwasm/team) group. You can find more info by visiting that repo!

It then uses a small NextJS webapp to demo the WASM functions we have written.

## Development: WASM

The WASM module was initialized as a lib. So it can not work on its own. If we have a look at the Cargo.toml inside the `./wasm` directory, we see that it is actually a `cdylib`; C dynamic library.

We can create a dev build of the package from the `./wasm` folder with:

```bash
wasm-pack build --target web --dev
```

Alternatively, just run `build-wasm.sh` to generate a release build targeting the web.

Building the module gives us our module, plus type definitions and a JavaScript wrapper inside the `pkg` folder.

> Remember to rebuild the package if you make any changes!

All the files for the library will be in the `./wasm/src` folder, and all generated WASM and supporting files will be inside the `./wasm/pkg/` folder. If you have a look in `tsconfig.js`, you can see that we have aliased this folder, so we can import with `@wasm/module-name`.

## Development: NextJS

First, run the development server:

```bash
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and load Inter, a custom Google Font.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!