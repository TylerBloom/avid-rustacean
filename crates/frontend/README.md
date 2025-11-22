## About
This crate contains a simple "Hello World" WASM app written in Yew.
You can freely substitute Yew of any other frontend framework that compiles to WASM, and this example will still work.

There are a couple of notable features of this crate. First and foremost, it is not part of the overall cargo workspace. Doing this causes a deadlock when compiling the `backend` (read more in that README). Second, there is a `.cargo` sub-directory. This forces there to be a target directory in this crate. This overwrites a user's global `cargo` config, so that if they are using a global target directory, it is not used for this project. We do this for cargo deadlock reasons. Lastly, and most importantly, there is an file called `index.html`. This is used by `trunk` as the basis for our app. You can insert your own CSS, links, and scripts to make your UI look as fancy as you wish.

## Setup
To build this, you will need the `wasm32-unknown-unknown` target (available via `rustup`).
While not necessary to compile this crate, you will need `trunk` if you want to fully run this project.

In the backend, `trunk` is used to compile this crate, generate the JS bridge code, and inject data into the HTML index file.

## Building
You can largely ignore building this crate directly.
If your app is simply enough, you can use `trunk server` for local testing; otherwise, you can indirectly build the `frontend` by building the `backend`.
