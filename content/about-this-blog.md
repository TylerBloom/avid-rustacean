+++
title = "About This Blog"
date = 2023-12-17

description = '''
As is my want, this blog is a fullstack Rust project. The backend is very straightforward as its job is largely to just serve blog post documents, and it has a fairly straightforward stack: [Axum](https://github.com/tokio-rs/axum) + [MongoDB](https://www.mongodb.com/) and deployed on [Shuttle](https://www.shuttle.rs/beta). All of the unique parts of the project are in the frontend. There, I combine  [Ratatui](https://github.com/ratatui-org/ratatui) and [Yew](https://github.com/yewstack/yew) for a texted-based, terminal-esque UI. This presented a series of fun and interesting challenges.
'''

[extra]
show_only_description = true

[taxonomies]
tags = [ "announcement" ]
+++

## TL;DR
As is my want, this blog is a fullstack Rust project. The backend is very straightforward as its job is largely to just serve blog post documents, and it has a fairly straightforward stack: [Axum](https://github.com/tokio-rs/axum) + [MongoDB](https://www.mongodb.com/) and deployed on [Shuttle](https://www.shuttle.rs/beta). All of the unique parts of the project are in the frontend. There, I combine  [Ratatui](https://github.com/ratatui-org/ratatui) and [Yew](https://github.com/yewstack/yew) for a texted-based, terminal-esque UI. This presented a series of fun and interesting challenges.

## Introduction
I'm on a mission with this blog. I want to change how people, especially other Rustaceans, think about Rust. I frequently see posts across social media platforms saying some variation of "I want to learn Rust, but I don't know what to build." To which many people recommend terminal applications, low-ish level projects, and web servers, all good and reasonable. But, at the heart of these questions and their responses, I think, is the idea that you should "use Rust as intended". After all, Rust is just a tool, and tools are geared to be used for a particular set of tasks.

The "intended usecases" of Rust is something that I plan to discuss a fair amount here because it shapes what we are willing to build (or at least willing to encourage get built). In all likelihood, you nor I nor (likely) any one person has a complete conception of what Rust is intended to solve, so I encourage you to explore a bit.

In my opinion, the ease with which you can render an idea into code (and visa versa) is what makes a language "good" for a particular application, and Rust excels at being expressive and portable. So, let's explore how far this idea can take us. I'll use this blog as a case study.

## The Vision
I've wanted to build a developer blog ever since learning Rust a bit more than 2 years ago. Building a blog is a rather simple task. Resources abound that help you build and deploy a blog. You can even do it directly through GitHub. While this sounds great, the general lack of constraint and my overall dislike of designing UIs made it hard to envision what I wanted for a long time. At least, until inspiration struck me: a TUI!

I practically live in the terminal and greatly enjoy the text-based aesthetic. My daily environment includes (neo)vim, i3, and a bevy of other terminal-based tool. So when I thought of this, I knew that I was going to have a great time build it. That said, a TUI that runs in your browser is clearly not an "intended usecase" for Rust. But why should I let that stop me? My rough idea at this point was to combine a web UI library with a TUI library and then praying that they could stick together somehow.

Well, you're reading this right now, so, clearly, it must of worked!

## The UI Stack
To start, I want to note that this is not a how-to or tutorial for any of the crates that I mention. They all have great examples and tutorials. If you feel inspired to build something after reading this (and I hope you do), start with their resources.

Since this blog is fullstack Rust, I knew that [trunk](https://github.com/trunk-rs/trunk) was a must. It auto generates all of the JS bindings your app needs as well as the HTML index file. For development, it can also watch your frontend crate for changes, compile it, and serve it. This makes for a great feedback loop while tweaking your code. Also, it allows you to forgo building a backend while you just trying to get a prototype working.

For the TUI part of the blog, I knew there were several libraries to help in rendering text-based UIs. I ended choosing [Ratatui](https://github.com/ratatui-org/ratatui)  because I knew it was relatively high level but almost nothing else. At that point, I was just hoping that I would be able to hack something together.

Once the text got rendered, it needed to be converted into HTML and served to the browser window. For this, I decided to use [Yew](https://github.com/yewstack/yew) because I had experience with it already.

With that, I was ready to start hacking around and hoping to come to some kind of revelation.

## Webatui
To say that the browser is not a normal terminal is an understatement. There is no `stdout`, so I needed a way to take the main renderer in Ratatui, the `Terminal`, and redirect any `stdout`-bound data somewhere else.

As luck would have it, the `Terminal` is generic over any type that implements their `Backend` trait. To start, I just wanted to get text in the browser, so I ignored everything that had to do with a cursor. This left me with a fairly short list of things that I needed to implement:
```rust
/// A simplified version of the Ratatui `Backend` trait
pub trait Backend {
    fn draw<'a, I>(&mut self, content: I) -> Result<()>
       where I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn clear(&mut self) -> Result<()>;
    fn size(&self) -> Result<Rect>;
    fn window_size(&mut self) -> Result<WindowSize>;
    fn flush(&mut self) -> Result<()>;
}
```

Most of these methods are fairly simple. The `size` and `window_size` methods are easily accessible properties of the browser's window, and the `clear` method speaks for itself.

The `draw` method hints at a fairly intuitive, if naive, understanding of how terminal displays work: "its *just* a grid of characters". So, let's run with that. The "web terminal" would be internally represented by a grid of characters (actually a grid of characters + formatting info, but you get my point).

Lastly, the `flush` method is were all the magic happens. According to the docs, `flush` is called to, well, flush any buffered contents to the terminal screen. In the browser, this would amount to converting the grid of characters into HTML. The HTML rendering is where Yew comes in.

## All About Yew
Yew is a reactive, Elm-inspired framework. It models pieces of the UI through its `Component` trait, which has a series of optional methods that but everything we need can be done with just three methods:

```rust
/// A simplified version of Yew's Component trait
pub trait Component: Sized + 'static {
    type Message: 'static;

    fn create(ctx: &Context<Self>) -> Self;
    fn view(&self, ctx: &Context<Self>) -> Html;

	// Technically "optional", but we'll need it.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool { ... }
}
```

Notice the return type of the `view` method, `Html`. Yew provides a series of tools for working with virtual HTML nodes (VNodes), namely the `html!` macro. Now, I generally avoid macros whenever possible (yet another things that I will provide write a whole post about), but `html!` is something special.

The `html!` macro allows you to, more or less, embed HTML in your Rust code, but that's underselling it. It allows you to embed HTML in your Rust code in the same way that the `format!` macro allows you to embed string literals in your Rust code.

```rust
// This produces a header node that can be displayed in your browser!
html! {
	<h1> { format!("Hello, {}!", self.name) } </h1>
}
```

This is how the custom "terminal" backend will render its grid of characters into HTML when `flush` is called.

## The First Render
With that, I had pieced enough stuff together to take a simple example from Ratatui and hopefully display it in the browser. So far, it has been a fairly simple process, and it looks like we'll have a relatively smooth sailing for now. I *just* need to: build a backend for the web terminal, put together a simple component for Yew to render, use that component to render a Ratatui example into the web terminal, and display the HTML rendered by the web terminal.

The initial terminal backend was simple to build. It used a series of `<pre>` tags to render the lines of text. Once rendered, it caches the HTML for reuse. This means that displaying the HTML just means getting a copy from the cache.

The component was even easier. To start, it carries no data and never updates. The one issue is that rendering a frame requires a mutable reference to the `Terminal` but `Component::view` only provides a shared reference to the component. No problem, this is what `RefCell` was built for (there are ways around this, but, again, we're prototyping).

With that, we're done! This works! We've married two random libraries that were certainly not designed to go together. We have a single component that represents the terminal screen, which can be expanded to hold any data that's needed. Yew has support for updating that component and for doing so asynchronously. All that's left is to throw together a backend to store and fetch data, and we're golden, right?

## The First Snag
No. Right now, we only support the barest intersection between terminals and browsers, displaying text. Unfortunately, most people expect to be able to interact with a web app especially with... the mouse.

Yew was built for the web, so it supports things like attaching callbacks to HTML tags for interactivity. Or even simpler, browser HTML supports hyperlinks. How can we add this kind of interactivity into the rendered HTML if Ratatui doesn't provide that data?

This stumped me, and I had it in the back of my mind for a couple of days. Then, it occurred to me. The `draw` method in the `Backend` takes an iterator over `Cell`s and their position. A `Cell` contains a character (technically a string but, for this usecase, a character) and some formatting information:
```rust
/// The Ratatui Cell
pub struct Cell {
    pub symbol: String,
    pub fg: Color,
    pub bg: Color,
    pub underline_color: Color,
    pub modifier: Modifier,
    pub skip: bool,
}
```

Most of this is very straightforward, but if part of the formatting data is unused, then it could be used to mark something as "in need of hydration" (in essence, it needs additional data before being rendered into HTML). From there, an additional step could be added to the rendering process in order to inject hyperlinks, callbacks, and other forms of interactivity.

Since I had decided to use the [base16](https://github.com/chriskempson/base16) approach to theming the blog, most of the color space is completely unused. A certain foreground/background color could be used as this flag, but this restricts the formatting options for interactable elements. Instead, I decided to use a part of the `modifier` field.

A `Modifier` is a bit field that communicates different formatting styles like marking text as needing to blink quickly or be reversed. There are several of these that I have no intention to use, let alone alone add support for. This avoids the clash from before, and exactly which modifier is used as the flag doesn't matter.

## Ending the Drought
With a system for flagging text as "needing hydration", the backend needs to change its rendering logic. It can't go directly from a grid of characters to HTML. Now, it needs to pause the rendering to allow for the component to provide additional data. This falls outside of the bounds of the Ratatui backend model, but that's fine. Ratatui has done its job well. It has rendered widgets into a field of text that will ultimately be displayed.

Instead of rendering the grid, `Backend::flush` will instead turn the grid into series of spans that note if it needs hydration. Something like this:

```rust
/// The spans used in the hydration process
enum TermSpan {
    /// The data is plain data and will be rendered in an HTML span tag.
    Plain((Color, Color), Modifier, String),
    /// The data might need to contain additional data, such as a callback.
    Dehydrated(DehydratedSpan),
}

/// A span that might need additional data such as a callback or hyperlink
#[derive(Debug, Default)]
pub struct DehydratedSpan {
    style: (Color, Color),
    mods: Modifier,
    text: String,
    // The container for the interactivity data, like the href for a hyperlink
    interaction: Interaction,
}
```

After the component has rendered everything via the Ratatui terminal renderer, it can call a method to hydrate those spans. This hydration will yield each dehydrated span to the component one-by-one and then render all the spans into HTML. The HTML is then accessed the same way as before.

And with that, we really do support everything we need to! You can click around the UI, change tabs, read blog posts, and use hyperlinks. Huzzah!
## Backend Basics
Ok, we're not quite done yet...

There still needs to be a place were we can fetch data from. A simple backend will suffice for this. Throw in a database for persistence, and it's mostly done. The backend just needs to support basic CRUD operations for the different types of content it holds (blog post, project summaries, homepage, etc).

But what is being posted? I write my notes, and by extension these posts, in [Obsidian](https://obsidian.md/), which is just a bunch of markdown files. Ideally, I could give the backend a markdown document, and it would serve those to the frontend. But, displaying rendered markdown in a terminal-friendly way requires parsing that markdown. Also, what about syntax highlighting? What kind of dev blog doesn't have syntax highlighted code snippets??

Ok, so maybe the backend needs to do more than *just* store and serve data. Luckily, there are already libraries for parsing markdown and for syntax highlighting, [markdown-rs](https://github.com/wooorm/markdown-rs) and [syntect](https://github.com/trishume/syntect) respectively. Not everything in markdown can be rendered into an ASCII display, so the backend parses out only the supported parts from the markdown syntax tree. Any code blocks in that markdown are then parsed, highlighted, and stored in a collection of spans. This pre-parsed minimal markdown AST is what is then served to the frontend when it fetches a blog post or the like. That markdown is then turned into Ratatui widgets and eventually displayed as the text that you are reading.

## Wrap Up
When I hear people talk about what Rust is built for, this kind of project is not discussed. This is understandable. This is an odd, somewhat esoteric way of using the language... but should it be?

I'm not advocating that people build other terminal-esque blogs (how else would I be unique?), but I do believe that you should chase your bless. While the crates that I used to build this were not intended to be used together, they come together relatively easily and quickly (about a week), but, more importantly, I had a blast while doing it.

Rust, far more than any language I've used, is a joy to work with, and I think developers should get to use things that bring them joy. You can bring Rust to almost any domain, and that's where I think you should write Rust code.

Well, its been a pleasure. Same time next time?
