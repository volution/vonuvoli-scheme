

<a id='appendix__r7rs__overview'></a>

# `r7rs` -- Overview


#### Description

> ##### Semantics
> 
> This section gives an overview of Scheme's semantics.  A
> detailed informal semantics is the subject of
> other sections.  For reference
> purposes, section on formal semantics provides a formal
> semantics of Scheme.
> 
> Scheme is a statically scoped programming
> language.  Each use of a variable is associated with a lexically
> apparent binding of that variable.
> 
> Scheme is a dynamically typed language.  Types
> are associated with values (also called objects) rather than
> with variables.
> Statically typed languages, by contrast, associate types with
> variables and expressions as well as with values.
> 
> All objects created in the course of a Scheme computation, including
> procedures and continuations, have unlimited extent.
> No Scheme object is ever destroyed.  The reason that
> implementations of Scheme do not (usually!) run out of storage is that
> they are permitted to reclaim the storage occupied by an object if
> they can prove that the object cannot possibly matter to any future
> computation.
> 
> Implementations of Scheme are required to be properly tail-recursive.
> This allows the execution of an iterative computation in constant space,
> even if the iterative computation is described by a syntactically
> recursive procedure.  Thus with a properly tail-recursive implementation,
> iteration can be expressed using the ordinary procedure-call
> mechanics, so that special iteration constructs are useful only as
> syntactic sugar.  See section on proper tail recursion.
> 
> Scheme procedures are objects in their own right.  Procedures can be
> created dynamically, stored in data structures, returned as results of
> procedures, and so on.
> 
> One distinguishing feature of Scheme is that continuations, which
> in most other languages only operate behind the scenes, also have
> "first-class" status.  Continuations are useful for implementing a
> wide variety of advanced control constructs, including non-local exits,
> backtracking, and coroutines.  See section on continuations.
> 
> Arguments to Scheme procedures are always passed by value, which
> means that the actual argument expressions are evaluated before the
> procedure gains control, regardless of whether the procedure needs the
> result of the evaluation.
> 
> Scheme's model of arithmetic is designed to remain as independent as
> possible of the particular ways in which numbers are represented within a
> computer. In Scheme, every integer is a rational number, every rational is a
> real, and every real is a complex number.  Thus the distinction between integer
> and real arithmetic, so important to many programming languages, does not
> appear in Scheme.  In its place is a distinction between exact arithmetic,
> which corresponds to the mathematical ideal, and inexact arithmetic on
> approximations.  Exact arithmetic is not limited to integers.
> 
> 
> ##### Syntax
> 
> Scheme, like most dialects of Lisp, employs a fully parenthesized prefix
> notation for programs and other data; the grammar of Scheme generates a
> sublanguage of the language used for data.  An important
> consequence of this simple, uniform representation is that
> Scheme programs and data can easily be treated uniformly by other Scheme programs.
> For example, the `eval` procedure evaluates a Scheme program expressed
> as data.
> 
> The `read` procedure performs syntactic as well as lexical decomposition of
> the data it reads.  The `read` procedure parses its input as data
> (section on external representation), not as program.
> 
> The formal syntax of Scheme is described in section on formal syntax.
> 
> 
> ##### Base and optional features
> 
> Every identifier defined in this report appears in one or more of several
> __libraries__.  Identifiers defined in the __base library__
> are not marked specially in the body of the report.
> This library includes the core syntax of Scheme
> and generally useful procedures that manipulate data.  For example, the
> variable `abs` is bound to a
> procedure of one argument that computes the absolute value of a
> number, and the variable `+` is bound to a procedure that computes
> sums.  The full list
> all the standard libraries and the identifiers they export is given in
> section on standard libraries.
> 
> All implementations of Scheme:
> 
>   * Must provide the base library and all the identifiers
> exported from it.
> 
>   * May provide or omit the other
> libraries given in this report, but each library must either be provided
> in its entirety, exporting no additional identifiers, or else omitted
> altogether.
> 
>   * May provide other libraries not described in this report.
> 
>   * May also extend the function of any identifier in this
> report, provided the extensions are not in conflict with the language
> reported here.
> 
>   * Must support portable
> code by providing a mode of operation in which the lexical syntax does
> not conflict with the lexical syntax described in this report.
> 
> 
> ##### Error situations and unspecified behavior
> 
> When speaking of an error situation, this report uses the phrase
> "an error is signaled" to indicate that implementations must detect and
> report the error.
> An error is signaled by raising a non-continuable exception, as if by
> the procedure `raise` as described in section on exceptions.  The object raised is implementation-dependent
> and need not be distinct from objects previously used for the same purpose.
> In addition to errors signaled in situations described in this
> report, programmers can signal their own errors and handle signaled errors.
> 
> The phrase "an error that satisfies __predicate__ is signaled" means that an error is
> signaled as above.  Furthermore, if the object that is signaled is
> passed to the specified predicate (such as `file-error?` or
> `read-error?`), the predicate returns `#t`.
> 
> If such wording does not appear in the discussion of
> an error, then implementations are not required to detect or report the
> error, though they are encouraged to do so.
> Such a situation is sometimes, but not always, referred to with the phrase
> "an error".
> In such a situation, an implementation may or may not signal an error;
> if it does signal an error, the object that is signaled may or may not
> satisfy the predicates `error-object?`, `file-error?`, or
> `read-error?`.
> Alternatively, implementations may provide non-portable extensions.
> 
> For example, it is an error for a procedure to be passed an argument of a type that
> the procedure is not explicitly specified to handle, even though such
> domain errors are seldom mentioned in this report.  Implementations may
> signal an error,
> extend a procedure's domain of definition to include such arguments,
> or fail catastrophically.
> 
> This report uses the phrase "may report a violation of an
> implementation restriction" to indicate circumstances under which an
> implementation is permitted to report that it is unable to continue
> execution of a correct program because of some restriction imposed by the
> implementation.  Implementation restrictions are discouraged,
> but implementations are encouraged to report violations of implementation
> restrictions.
> 
> For example, an implementation may report a violation of an
> implementation restriction if it does not have enough storage to run a
> program,
> or if an arithmetic operation would produce an exact number that is
> too large for the implementation to represent.
> 
> If the value of an expression is said to be "unspecified", then
> the expression must evaluate to some object without signaling an error,
> but the value depends on the implementation; this report explicitly does
> not say what value is returned.
> 
> Finally, the words and phrases "must", "must not", "shall",
> "shall not", "should", "should not", "may", "required",
> "recommended", and "optional", although not capitalized in this
> report, are to be interpreted as described in RFC 2119 [[RFC-2119]](#errors).
> They are used only with reference to implementer or implementation behavior,
> not with reference to programmer or program behavior.
> 
> 
> ##### Entry format
> 
> Sections are organized
> into entries.  Each entry describes one language feature or a group of
> related features, where a feature is either a syntactic construct or a
> procedure.  An entry begins with one or more header lines of the form
> 
> ````
> <template>    <category>
> ````
> 
> for identifiers in the base library, or
> 
> ````
> <template>    <name> library <category>
> ````
> 
> where `name` is the short name of a library
> as defined in section on standard libraries.
> 
> If `category` is `syntax`, the entry describes an expression
> type, and the template gives the syntax of the expression type.
> Components of expressions are designated by syntactic variables, which
> are written using angle brackets, for example `<expression>` and
> `<variable>`.  Syntactic variables are intended to denote segments of
> program text; for example, `<expression>` stands for any string of
> characters which is a syntactically valid expression.  The notation
> ````
> <thing_1> ...
> ````
> indicates zero or more occurrences of a `<thing>`, and
> ````
> <thing_1> <thing_2> ...
> ````
> indicates one or more occurrences of a `<thing>`.
> 
> If `category` is `auxiliary syntax`, then the entry describes a
> syntax binding that occurs only as part of specific surrounding
> expressions. Any use as an independent syntactic construct or
> variable is an error.
> 
> If `category` is `procedure`, then the entry describes a procedure, and
> the header line gives a template for a call to the procedure.  Argument
> names in the template are `italicized`.  Thus the header line
> 
> ````
> (vector-ref vector k)    procedure
> ````
> 
> indicates that the procedure bound to the `vector-ref` variable takes
> two arguments, a vector `vector` and an exact non-negative integer
> `k` (see below).  The header lines
> 
> ````
> (make-vector k)         procedure
> (make-vector k fill)    procedure
> ````
> 
> indicate that the `make-vector` procedure must be defined to take
> either one or two arguments.
> 
> It is an error for a procedure to be presented with an argument that it
> is not specified to handle.  For succinctness, we follow the convention
> that if an argument name is also the name of a type listed in
> section disjointness of types, then it is an error if that argument is not of the named type.
> For example, the header line for `vector-ref` given above dictates that the
> first argument to `vector-ref` is a vector.  The following naming
> conventions also imply type restrictions:
> 
>   * `alist` -- association list (list of pairs);
>   * `boolean` -- boolean value (`#t` or `#f`);
>   * `byte` -- exact integer `0 <= byte < 256`;
>   * `bytevector` -- bytevector;
>   * `char` -- character;
>   * `end` -- exact non-negative integer;
>   * `k, k_1, ... k_j, ...` -- exact non-negative integer;
>   * `letter` -- alphabetic character;
>   * `list, list_1, ... list_j, ...` -- list (see section pairs and lists);
>   * `n, n_1, ... n_j, ...` -- integer;
>   * `obj` -- any object;
>   * `pair` -- pair;
>   * `port` -- port;
>   * `proc` -- procedure;
>   * `q, q_1, ... q_j, ...` -- rational number;
>   * `start` -- exact non-negative integer;
>   * `string` -- string;
>   * `symbol` -- symbol;
>   * `thunk` -- zero-argument procedure;
>   * `vector` -- vector;
>   * `x, x_1, ... x_j, ...` -- real number;
>   * `y, y_1, ... y_j, ...` -- real number;
>   * `z, z_1, ... z_j, ...` -- complex number;
> 
> The names `start` and `end` are used as indexes into strings,
> vectors, and bytevectors.  Their use implies the following:
> 
>   * It is an error if `start` is greater than `end`.}
> 
>   * It is an error if `end` is greater than the length of the
> string, vector, or bytevector.}
> 
>   * If `start` is omitted, it is assumed to be zero.}
> 
>   * If `end` is omitted, it assumed to be the length of the string,
> vector, or bytevector.
> 
>   * The index `start` is always inclusive and the index `end` is always
> exclusive.  As an example, consider a string.  If
> `start` and `end` are the same, an empty
> substring is referred to, and if `start` is zero and `end` is
> the length of `string`, then the entire string is referred to.
> 
> 
> ##### Evaluation examples
> 
> The symbol `===>` used in program examples is read
> "evaluates to".  For example,
> 
> ````
> (* 5 8)      ===>  40
> ````
> 
> means that the expression `(* 5 8)` evaluates to the object `40`.
> Or, more precisely:  the expression given by the sequence of characters
> `(* 5 8)` evaluates, in the initial environment, to an object
> that can be represented externally by the sequence of characters
> `40`.  See section on external representations for a discussion of external
> representations of objects.
> 
> 
> ##### Naming conventions
> 
> By convention, `?` is the final character of the names
> of procedures that always return a boolean value.
> Such procedures are called __predicates__.
> Predicates are generally understood to be side-effect free, except that they
> may raise an exception when passed the wrong type of argument.
> 
> Similarly, `!` is the final character of the names
> of procedures that store values into previously
> allocated locations (see section on storage model).
> Such procedures are called __mutation procedures__.
> The value returned by a mutation procedure is unspecified.
> 
> By convention, `->` appears within the names of procedures that
> take an object of one type and return an analogous object of another type.
> For example, `list->vector` takes a list and returns a vector whose
> elements are the same as those of the list.
> 
> A __command__ is a procedure that does not return useful values
> to its continuation.
> 
> A __thunk__ is a procedure that does not accept arguments.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

