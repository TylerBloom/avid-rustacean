![GitHub Workflows](https://github.com/TylerBloom/avid-rustacean/actions/workflows/ar_ci.yml/badge.svg)
![Deployment](https://img.shields.io/endpoint?url=https%3A%2F%2Favid-rustacean.shuttleapp.rs%2Fapi%2Fv1%2Fbadge&label=Deployment)
![Maintenance](https://img.shields.io/badge/Maintenance-Actively%20Developed-brightgreen.svg)

## The Avid Rustacean
This repo contains all of the code for my blog, The Avid Rustacean.
The blog is fully Rust (front and back ends) and is split into three crates, `backend`, `model`, and `frontend`.

The backend uses Axum as its REST framework and MongoDB as its database.
When deployed, the backend's build script compiles the frontend assets (the HTML index, the WASM module, and the JS bridge code) and statically binds them into the server executable.

The frontend uses a combination of Yew and Ratatui to provide a TUI aesthetic in the broswer.
To see how this is done, check out [the article](https://avid-rustacean.shuttleapp.rs/blog/About-This-Blog) that I wrote about its creation.

Lastly, the model crate contains common code between the front and back ends (and a bit extra).
For example, the markdown AST definition is found here as well as the server-side parsing code and syntax highlighting code.

## Contributions
Feel free to make suggestions, bring up things that would make the site easier to use, or to fork this repo to make your own blog.
All that I ask is for derived works to also be AGPL-licensed.
The point of this project (both blog code and blog posts) is to share what I've created and thought about.
I ask that others approach this project with the same desire.

## Future Plans
Currently, this project stands is an MVP.
It works and is relatively easy to use (on desktop), but there is a lot of room for improvement.
I plan to extract out the Ratatui + Yew integration into its own crate eventually.
