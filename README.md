# egui & eframe docs

Friendly, example-driven documentation for [egui](https://github.com/emilk/egui) and eframe 0.36,
built with [Docusaurus](https://docusaurus.io).

Every Rust example in these docs is a real program that is compiled, run and
screenshotted by automated tests. The pages load their code straight from those
files, so the docs cannot show code that does not work.

## Read the docs locally

You need Node.js 20 or newer.

```bash
npm install
npm start
```

This opens the site at http://localhost:3000 and reloads when you edit a page.

To produce the static site (in `build/`) and preview it:

```bash
npm run build
npm run serve
```

The site is published to https://md-sakif.github.io/egui-docs/ by `.github/workflows/deploy.yml` on every push to `main`.

## What is where

| Path | What it is |
|---|---|
| `docs/` | The pages, as MDX |
| `sidebars.js` | The order of the pages in the sidebar |
| `examples/src/bin/` | One complete Rust program per example |
| `examples/tests/screenshots.rs` | The tests: they drive every example and take the screenshots |
| `examples/assets/` | Images and the font used by a few examples |
| `web-example/` | The project from the "Publishing for the web" page |
| `static/img/examples/` | Screenshots written by the tests |
| `src/components/` | `<Example>`, `<Screenshot>` and `<Card>`, used by the pages |

## Run the examples and the tests

You need Rust 1.95 or newer (`rustup update`).

```bash
cd examples
cargo run --bin widget_button      # run one example
cargo test --test screenshots      # test all examples and refresh the screenshots
```

The first build compiles a few hundred crates and takes several minutes. Because each
example is a full program, the `examples/target` folder grows to several gigabytes.
Delete it with `cargo clean` when you are done.

The web example needs [Trunk](https://trunkrs.dev) and the `wasm32-unknown-unknown` target:

```bash
cd web-example
trunk serve
```

## Add a page with an example

1. Write the program as `examples/src/bin/my_example.rs`.
2. Add a test for it to `examples/tests/screenshots.rs`:
   `example!(my_example, [320, 120], |_cc| MyApp::default());`
3. Run `cargo test --test screenshots my_example`. This writes
   `static/img/examples/my_example.light.png` and `.dark.png`.
4. Use it in a page: `<Example name="my_example" />` shows the code and the
   screenshot. `<Example name="my_example" item="fn ui" />` shows just one item.
5. Add the page to `sidebars.js`.

## Credits

- egui and eframe are by Emil Ernerfeldt and contributors, licensed MIT or Apache-2.0.
- The Lato font in `examples/assets` is by Łukasz Dziedzic, under the SIL Open Font
  License (see `examples/assets/Lato-LICENSE.txt`).
- The other images in `examples/assets` were generated for these docs.
