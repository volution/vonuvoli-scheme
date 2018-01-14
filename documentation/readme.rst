
###############
vonuvoli Scheme
###############


.. warning::  Work in progress

  Although the actual usage documentation (API, internals, etc.) is at the moment quite scarce, the `about <About_>`_ section is quite extensive in explaining what ``vonuvoli`` Scheme actually is all about, what is the current implementation status, how it differs from other Scheme implementations, and why it is written in Rust.


.. contents::
    :depth: 2



About
=====


.. sidebar::  About -- short version

  ``vonuvoli`` is a **Scheme R7RS**-based programming language with focus on **systems programming**, **extensibility** and **deployability**.

  Currently it runs only on UNIX-like operating systems, like **Linux** / OSX / BSD's / etc.  Porting it on Windows should be trivial, but is not currently a priority.


As the "short version" says it, ``vonuvoli`` is a Scheme_ based programming language implemented in Rust_, which supports most of the R7RS_ standard.




Why yet another Scheme / Lisp interpreter?
------------------------------------------

.. epigraph::  A.K.A.  That moment when everyone wonders about "another case of not-invented-here-syndrome"?


F.A.Q.  Why not reuse existing Scheme implementations?
......................................................

I have investigated reusing Chibi_ (a Scheme R7RS implementation) written in C, however it failed some primary requirements for the intended use-case.

Because many other C-based Scheme implementations seem to have the same issues, I'll quickly list Chibi's shortcomings bellow as highlights of what problems we are trying to avoid:

  * the runtime deployment isn't "untar-anywhere" compliant, because during the build process some paths get hard-coded into the binaries;
  * extending (by writing native libraries) in C is quite cumbersome, as much of the code is juggling `sexp_gc_var*`, `sexp_gc_preserve*`, `sexp_gc_release*` which are so easy to miss-use;
  * extending in C is also quite "unsafe" as one can easily misuse the various low-level value accessors with the wrong value type;
  * much of the builtin functionality is written in Scheme, which incurs quite an overhead during the VM initialization, and might miss some optimization opportunities;
  * at times it feels many features are abandoned experiments, most of which can be disabled at compile time, but incurring quite a lot of complexity;

Granted that Chibi (and other C-based Scheme implementations) are quite performant, feature-full, and more mature.


F.A.Q.  Why isn't Python / Ruby / Lua enough?
.............................................

Mainly because:

  * process management in Python / Ruby / Lua is quite cumbersome, especially when dealing with process pipelines;
  * lack of proper macros (i.e. syntax extensions) prohibits proper DSL creation, which makes some tasks cumbersome;
  * extending them with native libraries (i.e. in C) is quite involved;


F.A.Q.  Aren't there enough Scheme / Lisp?
..........................................

Apparently not.  Someone wrote somewhere that in the Scheme / Lisp world the norm is for more implementations per author than authors per implementation.  :)




Which are ``vonuvoli``'s primary focus areas?
---------------------------------------------

..

  * enabling easy **systems programming** (i.e. scripting), from **process** and **pipeline** management, **file-system** operations, and **inter-process** communication;
  * providing as **builtin functionality** various much needed **building-blocks** (like those related to cryptography, JSON, persistent key-value store, etc.), without hampering performance;
  * **minimizing the runtime footprint** in terms of files and dependencies, which enables ``tar``-based deployments (i.e. the runtime deployment should resume to ``mkdir /.../any-folder && cd /.../any-folder && curl http://.../vonuvoli.tar.gz | tar -xz``) and **relocatable** deployments (i.e. ``mv /.../old-folder /.../new-folder``);
  * **minimizing the runtime resource consumption** in terms of memory, enabling large in-memory datasets;  achieved mainly by using as few abstractions over Rust as possible;  (currently a Scheme value's overhead over its Rust native abstraction is only 8 octets, mainly due to alignment constraints;)
  * **performance** where it matters for the targeted use-cases;  which is achieved by implementing functionality as much as possible in Rust (thus compiled to native code), and providing as builtin functionality the most common patterns;
  * **extensibility** by enabling easy development of additional builtin functionality in Rust (and thus, again, compiled to native code);
  * **safety** by adhering to strict API contracts, providing "safe" building blocks (like immutable / mutable variants of strings, arrays, etc.), building upon Rust's `reference borrowing rules <RustBorrow_>`_ and `smart pointers <RustPointers_>`_, and in general favoring correctness over performance;




Which are ``vonuvoli``'s non-focus areas?
-----------------------------------------

..

  * computational performance --- if one needs high-performance algorithms, one can always write that code directly in Rust (or even C) and expose that as builtin functionality;
  * Windows portability --- as previously stated the development is mainly focused on UNIX-like operating systems, but porting it to Windows should be trivial building upon Rust's conditional compilation;
  * GUI and human interaction --- focusing mainly on systems programming, these matters should be better delegated to tools like ``dmenu`` or ``rofi``;
  * full Scheme R7RS compliance --- some of the "key" features of Scheme (mainly continuations) are sacrificed because they require heavy tradeoffs (especially in terms of performance and complexity) given the current implementation;  (this however might change;)  (for an up-to-date R7RS implementation status see `this report <./documentation/r7rs-support.md>`_;)




What is currently being worked on?
----------------------------------

.. epigraph::  A.K.A.  That section about "what features are currently missing, some of which are quite important and useful, but unfortunately of which 50% will be delayed forever --- unless someone steps-up, or even better pays the authors, to implement them"...

Scheme / Lisp related functionalities:

  * tail recursion --- this is one of the top TODO tasks;
  * Lisp ``defmacro``-like macros --- like tail recursion is at the top of the TODO list;
  * Scheme R7RS ``syntax-rules`` macros --- still a top TODO task, but much more involved than the simpler ``defmacro``-like counterparts;
  * Scheme R7RS ``define-record-type``;
  * Scheme R7RS ``error`` and related --- which is a low-hanging fruit in terms of implementation ease;
  * Scheme R7RS ``parametrized`` and related --- similar to ``error`` it should be trivial to implement;
  * Scheme R7RS ``dynamic-wind`` and related;
  * Scheme R7RS ``define-library`` and related;
  * Scheme R7RS ``eval`` and related;
  * Scheme R7RS ``delay`` and related;
  * (for an up-to-date Scheme R7RS implementation status see `this report <./documentation/r7rs-support.md>`_;)

Other builtin functionalities:

  * JSON functions and syntax;
  * regular expressions and syntax;
  * extended string / bytes / array / lists functions;
  * extended process management;
  * extended file-system operations;
  * cryptographic functions;




What is currently deferred?
---------------------------

.. epigraph::  A.K.A.  That section about "what features are currently missing, will be missing for the foreseeable future, and of which 100% will never be implemented"...

..

  * Scheme R7RS complex and rational numbers;
  * Scheme R7RS continuations (i.e. ``call/cc`` and related);
  * arbitrary precision numeric values;
  * (for an up-to-date Scheme R7RS implementation status see `this report <./documentation/r7rs-support.md>`_;)




Why Rust?
---------

Rust_ is a modern programming language, focusing on **performance**, **safety** and **systems programming**;  **compiled** via LLVM_ into native executables;  similar to C/C++ and Go;  actively developed by Mozilla and used in many mission-critical tools and software.

Writing the interpreter and builtins in Rust proved to be quite easy (compared to C/C++), most builtins being almost as concise as if written in Scheme.

Moreover given the plethora of Rust libraries available one can easily extend the interpreter with additional builtins.




Why not C/C++?
--------------

Simply put:

  * a nightmare to build;  (``autoconf``-and-company anyone?  perhaps ``CMake``?)
  * a nightmare to rely on other libraries;  (``rpm`` / ``apt`` / ``brew`` / ``latest-craze-package-manager`` anyone?)
  * nothing beats Rust's ``enum`` data-type, which is priceless in writing the interpreter;  in C one has to rely on ``union`` with an ``enum`` discriminator and hope no-one miss-types anything;  in C++ one has to rely on dynamic-casts, etc.;
  * nothing beats Rust's functions multiple return facility;  in C one has to rely on pointer arguments (which hopefully are non-``NULL``), and returning ``errno``-style values (which hopefully are checked and acted upon);
  * have I mentioned yet ``NULL``-pointer segmentation faults, double ``free``'s, ``\0``-terminated strings, uninitialized pointers, header files?  have I missed something?




Why not Go?
-----------

No tie-breaking advantage / disadvantage over Rust for this use-case.

Have I mentioned yet Rust's proper generics, proper macro system, ``enum`` data-type, proper dependency management, and native performance?




What does ``vonuvoli`` stand for?
---------------------------------

Nothing.  It's just a made-up word that has the following properties:

  * it's easy to remember, say, and type;
  * searching it on Google yields ``0`` exact matches, and only a ``10`` "similar word" results;




Documentation
=============




``vonuvoli`` Scheme interpreter
-------------------------------

Unfortunately currently there is no documentation about the interpreter invocation.
Basically the interpreter takes a proper Scheme source file and executes it.

However at the moment it doesn't support any flags, therefore its invocation is quite simple:

  ::

    vonuvoli-scheme-interpreter /.../script.ss

For example, executing all benchmark scripts:

  ::

    find ./tests/scripts -type f -name '*.ss' -exec ./target/debug/vonuvoli-scheme-interpreter '{}' \;




``vonuvoli`` Scheme compiler
----------------------------

Like with the interpreter, currently there is no documentation about the compiler invocation.
Basically the compiler takes a proper Scheme source file then compiles it and dumps the resulting ``Expression``.

However, just like with the interpreter, the invocation is quite simple:

  ::

    vonuvoli-scheme-compiler /.../script.ss

For example, compiling all benchmark scripts:

  ::

    find ./tests/scripts -type f -name '*.ss' -exec ./target/debug/vonuvoli-scheme-compiler '{}' \;




``vonuvoli`` Scheme tester and bencher
--------------------------------------

Like with the interpreter, currently there is no documentation about the compiler invocation.
Basically the tester and bencher take a proper Scheme test file and executes it.
(A "test" Scheme file is a simple syntax extension over "plain" Scheme: ``statement => expected-output``.)

However, just like with the interpreter, the invocation is quite simple:

  ::

    vonuvoli-scheme-tester /.../script.sst
    vonuvoli-scheme-bencher /.../script.sst

For example, testing all test-cases:

  ::

    find ./tests/scheme -type f -name '*.sst' -exec ./target/debug/vonuvoli-scheme-tester '{}' \;
    find ./tests/scheme -type f -name '*.sst' -exec ./target/debug/vonuvoli-scheme-bencher '{}' \;




``vonuvoli`` Scheme API
-----------------------

Unfortunately currently there is absolutely no documentation regarding the builtin functionality API.

However one can take a look at the `tests/scheme/*.sst <./tests/scheme>`_ files which provide good examples (expected inputs and outputs) for all the builtins.

Moreover one can look at the Scheme R7RS_ standard which is mostly implemented by this interpreter.
For an up-to-date Scheme R7RS implementation status see `this report <./documentation/r7rs-support.md>`_.




``vonuvoli`` Rust API
---------------------

Unfortunately currently there is no documentation about the Rust API.

However the code is quite simple, the type and function identifiers are quite self-explanatory, and one can just take a closer look.

Moreover, given that we are using Rust, one can't make any mistake which the compiler won't point out.




Architecture (i.e. how does it work?)
-------------------------------------


The interpreter is composed of multiple sub-systems, each focused on one single concern.


The ``Value`` and related types
...............................

The ``Value`` data-type is the object juggled all over the place.
It is an Rust ``enum`` data-type (i.e. a C-like tagged ``union``) which holds one variant per supported data-type.

Its implementation (and its related types implementations) can be found in the `sources/values_*.rs <./sources>`_ files.


The "builtins" functions
........................

These are plain Rust functions that receive ``Value``'s, check if the input arguments are of the right type, execute their functionality, and return.

Their implementation can be found in the `sources/builtin_*.rs <./sources>`_ files.


The "primitives" exposed to Scheme code
.......................................

These are Rust ``enum``'s that are exposed to the Scheme code as ``Value``'s and which are used to dispatch the matching "builtin" function.

Their implementation can be found in the `sources/primitives_*.rs <./sources>`_ files.


The ``Expression`` and related types
....................................

As opposed to many naive Scheme implementations (i.e. S-expression-based evaluators), and unlike the "stack"-based VM Scheme implementations (i.e. opcode-based evaluators), this implementation uses an AST-like approach, by defining a set of expression objects that can be evaluated.
These expression objects are embodied by the ``Expression`` Rust ``enum`` data-type.

One can easily observe there are quite a few variants, but many of these are just specializations of a more generic form, which help with evaluation performance.

The implementation can be found in the `sources/expressions.rs <./sources/expressions.rs>`_ file.


The compiler (``Value`` -> ``Expression``)
..........................................

The compiler (found in `sources/compiler.rs <./sources/compiler.rs>`_), as its name states, transforms the S-expression ``Value``'s into the most generic ``Expression``'s (i.e. without regard to optimizations).


The optimizer (``Expression`` -> ``Expression``)
................................................

The optimizer (found in `sources/compiler_optimizer.rs <./sources/compiler_optimizer.rs>`_), as its name states, takes a "generic" ``Expression`` and tries to transform it into a much more "specific" (but semantically equivalent) variant.

For example the following are just a few optimization examples:

  * ``(begin (begin (begin (+ 1 2)))`` is transformed to ``3``;
  * ``(if #t (something) (whatever))`` is transformed to ``(something)``;


The evaluator (``Expression`` -> ``Value``)
...........................................

The evaluator (found in `sources/evaluator.rs <./sources/evaluator.rs>`_), as its name states, evaluates an ``Expression`` to obtain a ``Value``.

Its code is quite trivial and does little else than dispatching to the various "builtins".




Adaptability (i.e. can it handle more than Scheme?)
---------------------------------------------------

Like many other Scheme implementations, it could implement (efficiently) almost any non-object-oriented programming language.

Therefore if one dislikes all the parentheses involved in Scheme / Lisp languages, one could easily write an alternative compiler.




Installation
============


Download binaries
-----------------


.. warning:: No binaries available yet!




Build from sources
------------------


Fetch the project source code
.............................

::

  git clone https://github.com/cipriancraciun/vonuvoli-scheme.git
  cd ./vonuvoli-scheme


Install Rust and Cargo (nightly version)
........................................

The snippets bellow describe a "manual" ``rustup`` deployment method, one which has zero side-effects on your system.
(The "official" `procedure <rustup-quick_>`_ implies a global per-user ``rustup`` deployment.)

(In the snippets bellow replace ``x86_64-unknown-linux-gnu`` with the variant matching your operating system available `here <rustup-manual_>`_.)

::

  mkdir -- ./.rust ./.rust/rustup ./.rust/cargo
  curl -s -o ./.rust/rustup-init.tmp -- https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
  mv -n -T -- ./.rust/rustup-init.tmp ./.rust/rustup-init
  chmod +x -- ./.rust/rustup-init

::

  export -- RUSTUP_HOME="${PWD}/.rust/rustup"
  export -- CARGO_HOME="${PWD}/.rust/cargo"
  export -- PATH="${PWD}/.rust/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin:${PWD}/.rust/cargo/bin:${PATH}"

::

  ./.rust/rustup-init -y --no-modify-path
  ./.rust/cargo/bin/rustup install nightly


Build the project in debug mode (optional step)
...............................................

::

  cargo build


Test the project in debug mode (optional step)
..............................................

::

  cargo test


Build the project in release mode
.................................

::

  cargo build --release


Deploy the binaries
...................

The following binary is the only one required to execute Scheme script.

::

  cp ./target/release/vonuvoli-scheme-interpreter /.../vonuvoli-scheme-interpreter

The following binaries are optional to see how Scheme scripts are translated into ``Expression`` objects, and to execute test cases.

::

  cp ./target/release/vonuvoli-scheme-compiler /.../vonuvoli-scheme-compiler
  cp ./target/release/vonuvoli-scheme-tester /.../vonuvoli-scheme-tester
  cp ./target/release/vonuvoli-scheme-bencher /.../vonuvoli-scheme-bencher




Authors
=======

Ciprian Dorin Craciun
  * `ciprian@volution.ro <mailto:ciprian@volution.ro>`_ or `ciprian.craciun@gmail.com <mailto:ciprian.craciun@gmail.com>`_
  * `<https://volution.ro/ciprian>`_
  * `<https://github.com/cipriancraciun>`_




Notice (copyright and licensing)
================================

.. sidebar::  Notice -- short version

    The code is licensed under LGPL 3 or later.

    Thus you can use this code without releasing your own code as open-source.
    However if you change the code within this repository you'll have to release it as per LGPL.

For details about the copyright and licensing, please consult the `notice <./documentation/licensing/notice.txt>`__ file in the `documentation/licensing <./documentation/licensing>`_ folder.

If someone requires the sources and/or documentation to be released
under a different license, please send an email to the authors,
stating the licensing requirements, accompanied with the reasons
and other details; then, depending on the situation, the authors might
release the sources and/or documentation under a different license.




References
==========

.. [Scheme] `Scheme @WikiPedia <https://goo.gl/Bcg7bH>`_
.. [R7RS] `Revised 7th Report on the Algorithmic Language Scheme (R7RS) <https://goo.gl/5Ye5MU>`_

.. [Rust] `Rust (home page) <https://goo.gl/Vs6vNc>`_
.. [RustBorrow] `Rust (documentation) -- References and Borrowing <https://goo.gl/eejsYR>`_
.. [RustPointers] `Rust (documentation) -- Smart Pointers <https://goo.gl/teuMYS>`_

.. [rustup-quick] `rustup (tool) -- quick install method <https://goo.gl/SpGgti>`_
.. [rustup-manual] `rustup (tool) -- manual install method <https://goo.gl/vxABrt>`_

.. [LLVM] `LLVM Compiler Infrastructure (home page) <https://goo.gl/QRHTjB>`_

.. [Chibi] `Chibi Scheme (home page) <https://goo.gl/T26w5X>`_

