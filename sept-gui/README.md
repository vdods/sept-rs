# `sept-gui`

GUI editor for sept data.

## Notes

-   Layouts in `egui` don't work exactly as expected (e.g. composite widgets don't get centered because of limitations in single-pass, immediate mode GUI).  This means that workarounds will have to be done sometimes, where UI layout that isn't exactly what's desired is used.   See https://github.com/emilk/egui/issues/843

## To-dos


## To-don'ts (i.e. Done)


## Build, Run, Deploy (these are instructions from `eframe_template`)

### Building and Running Native Locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel fontconfig-devel`

### Building and Running Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install Trunk with `cargo install --locked trunk`.
2. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
3. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy

1. Just run `trunk build --release`.  In order to make a build that is installable at a URL that isn't the root directory, you'll need to specify `--dist <dirname> --public-url <dirname>` and then copy `<dirname>` to your website.
2. It will generate a `dist` (or `<dirname>`) directory as a "static html" website
3. Upload the `dist` (or `<dirname>`) directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.
> To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
>
> If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.

You can test the template app at <https://emilk.github.io/eframe_template/>.
