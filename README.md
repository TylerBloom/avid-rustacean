![GitHub Workflows](https://github.com/TylerBloom/avid-rustacean/actions/workflows/ci.yml/badge.svg)
![Deployment](https://img.shields.io/endpoint?url=https%3A%2F%2Favid-rustacean.shuttleapp.rs%2Fapi%2Fv1%2Fbadge&label=Deployment)
![Maintenance](https://img.shields.io/badge/Maintenance-Actively%20Developed-brightgreen.svg)

## The Avid Rustacean
This repo contains all of the code for my blog, The Avid Rustacean.
The blog is fully Rust (front and back ends) and is split into three crates, `backend`, `model`, and `frontend`.

The backend uses Axum as its REST framework and MongoDB as its database.
When deployed, the backend's build script compiles the frontend assets (the HTML index, the WASM module, and the JS bridge code) and statically binds them into the server executable.

The frontend uses a combination of Yew and Ratatui to provide a TUI aesthetic in the browser.
To see how this is done, check out [the article](https://avid-rustacean.shuttleapp.rs/blog/About-This-Blog) that I wrote about its creation.

Lastly, the model crate contains common code between the front and back ends (and a bit extra).
For example, the markdown AST definition is found here as well as the server-side parsing code and syntax highlighting code.

## Contributions
Feel free to make suggestions, bring up things that would make the site easier to use, or to fork this repo to make your own blog.
All that I ask is for derived works to also be AGPL-licensed.
The point of this project (both blog code and blog posts) is to share what I've created and thought about.
I ask that others approach this project with the same desire.

## Development
If you'd like to run a local version of this project for development, there are a few requirements.

First, let's get the backend compiling.
The backend uses [Shuttle](https://www.shuttle.rs/beta) for deployment, which has a local deployment option.
Install shuttle via cargo with `cargo install cargo-shuttle`.
Shuttle uses docker, so make sure that it is installed and running.

Next, the frontend.
You'll need to make sure you have the `wasm32-unknown-unknown` target installed via `rustup` (or however you manage your Rust toolchains).
For local deployments, you will need [Trunk](https://trunkrs.dev/), which you can install via cargo with `cargo install trunk`.

For running everything, you'll launch the front and back ends separately.
In the `backend` directory, simply run `cargo shuttle run`.
This will compile and run the backend server, which will listen to requests on port 8000.
If you change the backend, you will need to re-run this.
In the `frontend` directory, run `trunk serve`.
Trunk acts like a frontend proxy that serves the frontend assets and proxies any other calls to the backend.
The `Trunk.toml` file contains this config.
By default, Trunk serves assets on port 8080.
If you make code changes to the frontend, Trunk will automatically recompile and re-deploy the frontend without you needing to reload the page.

To access the app on the machine that's deploying it, simply go to `http://localhost:8080`, and you'll see it.
If you'd like to access it from another device (like a phone), make sure both the host and client devices are on the same network.
When you launch Trunk, you'll see a few output like this:
```
    🏠 http://127.0.0.1:8080
    💻 http://192.168.202.218:8080
    💻 http://172.0.0.1:8080
```
The middle one contains your local IP address.
On the client device, go to that address and you'll see your app.

## Future Plans
Currently, this project stands is an MVP.
It works and is relatively easy to use (on desktop), but there is a lot of room for improvement.
