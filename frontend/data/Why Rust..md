Date: 2023-11-27 14:32

type: #post

# TL;DR
This post serves as an introduction to the blog, The Avid Rustacean, as well as me as a developer/engineer. The article will be broken up into 4 parts/arguments.
- Why I use Rust in my projects, particular when it is not required
- What I think the Rust community gets wrong
- "Why not Rust"
- A final synthesis of the "real" reasons

With maybe the exception of the last part, I will not be making a general argument for why Rust is the right choice.

# Citations
SO dev survey - Rust is loved

# Notes
Why I use Rust for personal work:
Because I like it. At least when I started working with Rust for personal work, that was the final reason. Its was enjoyable to use. I hope to dissect why that is the case here, but I think that, as technical people, we don't give our enjoyment enough weight. If you will find yourself mired in a project for a long time, you deserve to enjoy that time. Particularly for personal work (but also professional), don't make yourself miserable. This cuts both ways. If you have given Rust a fair shake, don't feel obligated to use it. That probably won't be the case (see SO survey), but there is that 10%.

## What the community gets wrong
Rust is oft touted for its speed and safety, and these things are both important aspects to the language. At the same time, they are perhaps the worst things to headline your argument for its adaption.

Consider your audience. You can loosely group folks into three language camps: C/C++, JS/Python, and Java/Go. To the C/C++ group, the first point is moot and the second "can be mitigated". To the JS/Python group, the second point is moot and their language is likely "fast enough for my needs" (which is likely true). To the last group, they have both of those things. Try again.

Frankly, both of those features being the face of your language paired with an unflinching fervor can, from an outside perspective, be... trying. Moreover, speed and safety only are the worst grounds on which to sell the language. Of course, they are key to Rust's success, but, to most, "not slow and won't segfault" is the baseline. To be clear, Rust rises far from that bar, but it is difficult to fully conceptualize that without hands-on experience with Rust, the very thing that we are trying to convince people to try out. So, what do I think works.

From my experience and what I've heard a chorus of people say, usability, longevity, and stability are the strongest reasons. Stability is the most straightforward. You program is very unlike to crash. The exact reasons for this will be subjects of future blog posts, so I'll sum it up as this: Rust makes solid code enjoyable and easy to write. Its that simple.

Of this list, usability, particularly tooling, is the most cited reason why people love the language. Cargo and its surrounding ecosystem make writing, maintaining, and using libraries a breeze. Of course, this ease of this interoperation of third-party code is only easy because of language-level features (universal ownership model, lifetimes, etc) but, again, that's a topic for a different day. Compared to nearly all other mainstream languages, Rust's toolchain is phenomenally easy to use without sacrificing too much. 

Unlike the other reason, the longevity/maintainability of the Rust code is a bit harder to grasp from an outside perspective. After all, why should Rust be easier to maintain than say, Java or C++. They are all statically typed.... ADTs and traits + reasonable, universal standards (again, ownership system + borrowing). Together, this allows devs to render intention clearly while also putting up guard rails that will guide users to the current usage. "Not, that type is not Send", "The variable was moved during this function call", "The given value might not life long enough". It is important to remember that we are often the more frequent users of our creations, and Rust gives us a way to remember our old reasonings without loosing a foot in the process.

I think its important as make a case for when Rust isn't a good choice. Since profession usecases vary widely, I'll keep those arguments short. A re-write is a massive endeavour. While you can incrementally add Rust to a project (often via FFI), this can be very tricky and adds to the front-loaded learning curve of Rust. (I will have a standalone post [[On the Rust learning curve]] at some point.) While there are times that adding Rust isn't difficult, such as with a mirco-service architecture, this is far from universally applicable. While I believe that Rust's features can benefit most (if not all) companies in the long rust, adopting it is not free and companies should be slow to move away from technology that they know works well for them.

For personal projects, there are more valid reasons to not use Rust than invalid ones. As I stated before, I love Rust and that was the deciding factor for me pursuing it personally and professionally. If there is a stack that you enjoy, your enjoyment of that workflow is enough of a reason for you to use it for your own projects. Moreover, there is often a pressure in dev circles to always be on the bleeding edge or to try out whatever is being hyped. If your interest in something has not been sparked or you don't have the time to learn something. Don't. Magically, the world will continue to spin, and you will not lose your job.

I want to close this section with something I alluded to: invalid reasons to not use Rust. In chronically-online speak, these can be summed up as "coping". Let's use C++'s toolchain and safety features as an example. A common reason that I see people online saying that they will continue to use C++ over Rust is that "C++ can be just as safe as Rust". This is because unhelpfully true and provably false. While, yes, Rust and C++ can be used to compile an instruction-by-instruction identical binary, or just C++ without any UB or access violation, saying what C++ **can** is unhelpful. Rather, 

# Draft


# Write Up
