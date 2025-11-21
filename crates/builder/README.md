## About
This crate is a crux of the project. Compiling and running the `backend` compiles serves the entire project. While this crate uses the `axum` framework, this template is backend-agnostic. You can free use any framework you would like, just adjust the API methods and router.

## Building
As part of building the `backend`, the `frontend` is built as well. This requires that you have both the `wasm32-unknown-unknown` target installed as well as `trunk`. These are avaiable via `rustup` and `cargo install`, respectively.

The build script for this crate will use `trunk` to compile the `frontend`, generate the JS bridge code, and inject some data into the `index.html` found in the `frontend`. These three build artifacts are placed in the `assets` directory at the root of the project. Each of these is then included via the `include_str!` or `include_bytes!` macro and bundled into the final executable. Bundling these assets like this removes the need to mess with the file system since we need these artifacts in order to deploy the app and we already know exactly where they are.

Building the `backend` this way does require some considerations. Namely, creating a compile-time deadlock with `cargo`. Building this crate requires that (at some level) `cargo build` be ran. As part of that build process, `cargo` builds and runs the build script, which the calls `cargo build` (via `trunk`) on the `frontend`. Because `cargo` is responsible, it uses lock files inside the target directory to avoid corrupting the build environment. Somewhat unfortunately, this requires that the `frontend` and `backend` crates never share a target directory. This is something to be mindful of if you want to edit the layout that this template presents.

## Deployment
To deploy the app (either locally or on Shuttle), all you need to do in run this crate.

A note on the build script for deploying on Shuttle: It uses a hack that you can [read more about on the main shuttle repo](https://github.com/shuttle-hq/shuttle/issues/703#issuecomment-1515606621). This requires fetching (and building) external dependencies, like `trunk`, in the build script. When deploying, you will notice a long pause (several minutes) late into the compilation of the `backend`. This is caused by needing to build `trunk` from scratch as well as the `frontend`.

Ideally, Shuttle will have first-case support for things such as `cargo binstall` and `trunk`. Until then, this part of the build script will have to suffice; however, there is no getting out the fact that the deployment process will require a clean build of the frontend.
