+++
title = "Projects"
path = "projects"
+++

# Specter
Specter is my newest project and is still in the very early phases of development. Specter is my attempt as emulators of Nintendo handhelds (particularly the GameBoy series). My plan for Specter is to provide an interconnected service for syncing save data and to act as a personal repository of ROMs.

Like everything I write, this project will be pure Rust, with a WASM web frontend, native Rust desktop and mobile apps, and even a handheld. The current plans for the handheld is to use a Raspberry Pi as the core and work from there; however, other chip options are being considered.

You can find the project repo [here](https://github.com/TylerBloom/specter).

# Avid Rustacean and Webatui
This blog is one of my newest projects. After publishing this blog, I split off all of the rendering logic into a separate crate called Webatui, which contains the integration between Ratatui and Yew.

I have several major goals for this blog. The largest of these is to jump start my contributions to the pool of Rust knowledge and education. What I hope to be the flagship series of blog posts, Rust from First Principles, is my attempt to explain the rules of Rust from the prespective of designing a language which forces you to write a "correct" program. An off-shoot of this is a series of posts about what I'm working on, be it this blog or any of my other projects. I will also be talking about what ever ideas I find interesting. Lastly, all of these will sum of up a public archive what I've been thinking about.

I hope you enjoy your time here!

# Squire
Squire was my first "real" project and is now archived. It was a fantastic learning expierence and was formative in becoming an developer.

Squire is tournament software for running Magic: the Gathering tournaments and designed to allow for maximum flexibility for tournament organizers.

Originally a Discord bot written in Python, this project sparked my learning of Rust after the Discord API library it used was deprecated. The bot was rewritten in Rust and then generalized into a full webservice. It has been a tremendous learning opportunity for me and many others.

Squire uses full stack Rust, with Axum in the backend and Yew in the frontend. Work on mobile apps has recently started with a planned stack of Yew and Tauri. A similar stack will be used for the native desktop app.

# Troupe
Troupe is an actor model framework built on top of Tokio. It sprung from my work on Squire as a way of modelling state both the front and back ends of that project. Because of this, Troupe is designed to work well for both native and WASM targets.

For WASM, the tokio runtime isn't used. Instead, the browser-produced runtime (accessible via [wasm-bindgen-futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/index.html)) is used, but tokio channels are still used for message passing.

You can check out the project repo [here](https://github.com/TylerBloom/troupe).
