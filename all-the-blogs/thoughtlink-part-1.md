---
layout: post
title: "ThoughtLink Part 1"
---

# Getting started with Foundation

I have a web project I want to make. And like many of my projects, I
figure I'll write down what I'm doing, get some blog karma. Whatever.

The pitch for this project, "ThoughtLink Assistant":

> A tool based on the principles of cognitive behavioral therapy,
  ThoughtLink Assistant presents a simple chat-like interface through
  which the user enters their _distressing thoughts_; and it helps
  them _untangle_ those thoughts using a variety of techniques, in
  order to reduce their _negative emotions_.

This project will initially run entirely on the client, requiring no
server-side support besides loading of static assets. Eventually it
will be able to create accounts and sync data to the server. It is
intended to be packagable as a local "fat" application with
[electron], or similar technology.

As a non-web-developer, the web stack is intimidating. Plenty has been
written about the massive number of options. There's so much to learn
before one can accomplish anything.

The first big choice one must make when starting a client-side web
project is which CSS framework to use. Options range from simple [CSS
resets] and all custom CSS, to simple grid systems like [Skeleton] to
the well-known, full-blown frameworks of [Bootstrap] and [Foundation].

[CSS resets]: https://yuilibrary.com/yui/docs/cssreset/
[Skeleton]: http://getskeleton.com/

I have some experience with Bootstrap, and for this project I compared
it with Foundation. I decided to go with Foundation because I liked
it's "mobile first" design, and that it uses rem for units instead of
pixels. Something new to learn.

I am a little worried about the complexity of Foundation - reading the
website is pretty damned overwhelming for something that basically
provides a set of CSS rules; and that the website pushes so hard to
invest in "training", that so much of the teaching material is in the
form of videos. This is a product made by a consulting company and it
shows big time. Sets off alarms for me, but let's see what happens.

There are - of course, this is web programming - numerous ways to
install foundation. Their preferred way seems to be to use `npm` to
install the `foundation-cli` tool, which can be used to generate
foundation projects. That's right, to generate CSS we need a custom
command line tool. Foundation uses [SASS], which has to be converted
to CSS for final delivery, and it seems like the `foundation-cli` tool
mostly does is help create an `npm` project that does the SASS
compilation correctly. I don't know, god damn, I want to quit already.

## Preparing the environment for Foundation

I'm tempted to go the easy route and use the precompiled CSS, but I
might as well learn something and do it the recommended way, so

```sh
sudo npm install --global foundation-cli
```

And now I want to create a foundation project with `foundation new`:

```sh
$ foundation
/usr/bin/env: ‘node’: No such file or directory
```

Oh, of course this is a Debian-based distro ([WSL] actually), so
the `node` binary is called `nodejs`. Fucking computers. I've dealt
with this before. So I'm going to symlink `/usr/bin/node` to `/usr/bin/nodejs`:

```sh
brian@DESKTOP-UCV672I:/mnt/c/dev/thoughtlink$ sudo ln -s /usr/bin/nodejs /usr/bin/node
```

Now I can run the Foundation cli! Right?

```sh
$ foundation new
/usr/local/lib/node_modules/foundation-cli/lib/commands/info.js:7
module.exports = function(args = {}) {
                               ^

SyntaxError: Unexpected token =
    at exports.runInThisContext (vm.js:53:16)
    at Module._compile (module.js:374:25)
    at Object.Module._extensions..js (module.js:417:10)
    at Module.load (module.js:344:32)
    at Function.Module._load (module.js:301:12)
    at Module.require (module.js:354:17)
    at require (internal/module.js:12:17)
    at Object.<anonymous> (/usr/local/lib/node_modules/foundation-cli/lib/index.js:4:9)
    at Module._compile (module.js:410:26)
    at Object.Module._extensions..js (module.js:417:10)
```

No, of course not. Computers, again.

I have node.js v4.2.6, and apparently Foundation needs > 6, so I
guess, like, fuck. I'll uninstall the system node and then download
them from [https://nodejs.org/en/download/].

```sh
$ sudo apt remove nodejs
```

I better delete that symlink I created too...

```sh
sudo rm /usr/bin/node
```

Now download the official bins.

```sh
$ curl -O https://nodejs.org/dist/v6.11.2/node-v6.11.2-linux-x64.tar.xz
$ tar xf node-v6.11.2-linux-x64.tar.xz node-v6.11.2-linux-x64/
```

And what does that give us?

```sh
 ls node-v6.11.2-linux-x64
bin  CHANGELOG.md  include  lib  LICENSE  README.md  share
```

A directory full of bins in standard Unix filesystem layout. What am I
supposed to do with that? Bleh. Oh, wait there are _also_
[instructions for installing on Debian/Ubuntu][inst]. Let's try that instead.

[inst]: https://nodejs.org/en/download/package-manager/#debian-and-ubuntu-based-linux-distributions

```sh
curl -sL https://deb.nodesource.com/setup_8.x | sudo -E bash -
```

That command (presumably, the instructions don't say shit) adds an apt
repo for an updated node.js and then updates the caches. Now finally we
can install the right (hopefully) node.js.

```sh
sudo apt install -y nodejs
```

And now?

```sh
$ node -v
v8.4.0
$ foundation --version
Foundation CLI version 2.2.3
```

Oh, yay. We've got a node.js that is compatible with the command line
application we are going to use to _set up our fucking CSS
files_. Holy shit, all those old emotions are flooding back. This is
why I hate computers.

## Creating the project

Now we can run `foundation new` and create the project. When I do so I
see this:


```sh
$ foundation new
? What are you building today? (Use arrow keys)
❯ A website (Foundation for Sites)
  A web app (Foundation for Apps)
  An email (Foundation for Emails)
```

Oh, neat, what's "Foundation for Apps"? I haven't seen that
anywhere. It sounds like exactly what I want, since I'm building a
local-first, mobile-first, single-page web application, so I select
it, enter the name of my project, "thoughtlink", and the Foundation
CLI starts downloading lots of stuff.

While it's working I DuckDuckGo for "Foundation for Apps", and find
its [documentation][d], where I see the following "deprecation
notice":

[d]: http://foundation.zurb.com/apps/docs/

> We believe the best solution for the future of the web is a single,
  robust framework capable of developing webapps and websites, so
  we’ve made the decision to streamline our development and move
  Foundation for Apps into our experimental playground to concentrate
  on Foundation for Sites. Foundation has, and will continue to push
  the web forward, and we’re incredibly excited about the future. You
  can follow along with the Foundation for Sites roadmap to get more
  details on where the project is headed and learn how to get
  involved.

Ok, so I guess that's another dead end. Use "Foundation for Sites",
not "Foundation for Apps".

After `foundation-cli` creates my "Foundation for Apps" directory I
immediately delete it and try again. This time I pick the correct type
of project ("Foundation for Sites", duh):

```sh
brian@DESKTOP-UCV672I:/mnt/c/dev$ foundation new
? What are you building today? A website (Foundation for Sites)
? What's the project called? (no spaces) thoughtlink
? Which template would you like to use? (Use arrow keys)
❯ Basic Template: includes a Sass compiler
  ZURB Template: includes Handlebars templates and Sass/JS compilers
```

And now it wants me to pick which template. I'm torn. Do I need the
handlebars templates? Do I need the JS compiler? What does JS compiler
even mean in this context? Is it babel? A minimizer? There's a description
of the difference [in the docs][zurb].

[zurb]: http://foundation.zurb.com/sites/docs/starter-projects.html

The ZURB template sounds pretty awesome: it uses webpack to compile
ES6 modules into a single JS file, which I think is something I'll
want to understand. I'm going to go all-in and do the ZURB template.

After it finishes the lengthy process of installing all the stuff I
need, `foundation` tells me "now run `foundation watch` while inside the
`thoughtlink` folder".

I'm suspicous of letting the `foundation` command control my life like
that, so I instead run `npm start`. And indeed, I have a very basic
static website running on `localhost:8000`. I wonder what value
`foundation watch` adds...

Now I have a directory that contains 535 MB of goodies, and
has the following structure:

- thoughtlink
  - .babelrc
  - .bowerrc
  - .gitignore
  - CHANGELOG.md
  - config.yml
  - dist/
    - assets/
      - css/
        - app.css
      - img/ (empty dir)
      - js/
        - app.js
      - scss/ (empty dir)
    - index.html
    - styleguide.html
  - etc/ (empty dir)
  - gulpfile.babel.js
  - node_modules
  - package.json
  - readme.md
  - src/
    - assets/
      - img/ (empty dir)
      - js/
        - app.js
	- lib/
	  - foundation-explicit-pieces.js
      - scss/
        - _settings.scss
	- app.scss
	- components/ (empty dir)
    - data/ (empty dir)
    - layouts/
      - default.html
    - pages/
      - index.html
    - partials/ (empty dir)
    - styleguide/
      - index.md
      - template.html

So that's a lot. I recognize most of it. Here are my guesses:

`package.json` is of course the npm configuration file. It should be
in control of everything. It mentions gulp a lot, so I need to learn
quickly wtf gulp does. It looks like the npm "start" and "build"
commands defer to gulp. `gulpfile.babel.js` is presumably the gulp
configuration. I'm guessing the `.babel.js` extension means it is run
through babel before execution. Not sure which component owns
`config.yml`, seems to contain configuration for several tools, but
I suspect it's a gulp configuration, oh, indeed it is mentioned in
`gulpfile.babel.js`. So gulp has two configuration files.

Ok, I need to learn what gulp is. Its website says "gulp is a toolkit
for automating painful or time-consuming tasks in your development
workflow, so you can stop messing around and build something". So it's
a build tool for web stuff. I guess it's what deals with babel, scss,
etc.

Moving on, obviously the `src` directory is transformed by gulp into
the `dist` directory. Contents of `src/assets` is pretty
straightforward. `src/layouts` is going to be handlebars templates
that the pages in `src/pages` will be rendered through. Don`t know
what `src/partials` is. `src/styleguide` is some ZURB designer
bullshit. I'll learn to appreciate it later.

Well.

I feel like I'm _really far down the rabbit hole_ now, and I have
written no code whatsoever. Now I'm invested in node.js, gulp, bower
(whatever that is - I've seen it scrolling by), foundation-cli, sass,
handlebars, webpack, PhantomJS (holy shit why did this install
PhantomJS...), a bunch of image minification libraries.

I hope I'm going in the right direction... This is an offline
application that deploys only static assets, and it's build process is
already shockingly dynamic. Feels bad.

I was hoping to prototype a screen or two today, but 3 hours later and
I'm just going to commit this monster and go drinking instead.