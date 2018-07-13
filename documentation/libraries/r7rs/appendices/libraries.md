

<a id='appendix__r7rs__libraries'></a>

# `r7rs` -- Libraries


<a id='appendix__r7rs__libraries__description'></a>

#### Description

> Libraries provide a way to organize Scheme programs into reusable parts
> with explicitly defined interfaces to the rest of the program.  This
> section defines the notation and semantics for libraries.
> 
> 
> ###### Library Syntax
> 
> A library definition takes the following form:
> 
> ````
> (define-library <library name>
>   <library declaration> ...)
> ````
> 
> `<library name>` is a list whose members are identifiers and exact non-negative integers.  It is used to
> identify the library uniquely when importing from other programs or
> libraries.
> Libraries whose first identifier is `scheme` are reserved for use by this
> report and future versions of this report.
> Libraries whose first identifier is `srfi` are reserved for libraries
> implementing Scheme Requests for Implementation.
> It is inadvisable, but not an error, for identifiers in library names to
> contain any of the characters `| \ ? * < " : > + [ ] /`
> or control characters after escapes are expanded.
> 
> A `<library declaration>` is any of:
> 
>   * `(export <export spec> ...)`
>   * `(import <import set> ...)`
>   * `(begin <command or definition> ...)`
>   * `(include <filename_1> <filename_2> ...)`
>   * `(include-ci <filename_1> <filename_2> ...)`
>   * `(include-library-declarations <filename_1> <filename_2> ...)`
>   * `(cond-expand <ce-clause_1> <ce-clause_2> ...)`
> 
> An `export` declaration specifies a list of identifiers which
> can be made visible to other libraries or programs.
> An `<export spec>` takes one of the following forms:
> 
>   * `<identifier>`
>   * `(rename <identifier_1> <identifier_2>)`
> 
> In an `<export spec>`, an `<identifier>` names a single
> binding defined within or imported into the library, where the
> external name for the export is the same as the name of the binding
> within the library. A `rename` spec exports the binding
> defined within or imported into the library and named by
> `<identifier_1>` in each
> `(<identifier_1> <identifier_2>)` pairing,
> using `<identifier_2>` as the external name.
> 
> An `import` declaration provides a way to import the identifiers
> exported by another library.  It has the same syntax and semantics as
> an import declaration used in a program or at the REPL (see section on `import`).
> 
> The `begin`, `include`, and `include-ci` declarations are
> used to specify the body of
> the library.  They have the same syntax and semantics as the corresponding
> expression types.
> This form of `begin` is analogous to, but not the same as, the
> two types of `begin` defined in section on sequencing.
> 
> The `include-library-declarations` declaration is similar to
> `include` except that the contents of the file are spliced directly into the
> current library definition.  This can be used, for example, to share the
> same `export` declaration among multiple libraries as a simple
> form of library interface.
> 
> The `cond-expand` declaration has the same syntax and semantics as
> the `cond-expand` expression type, except that it expands to
> spliced-in library declarations rather than expressions enclosed in `begin`.
> 
> One possible implementation of libraries is as follows:
> After all `cond-expand` library declarations are expanded, a new
> environment is constructed for the library consisting of all
> imported bindings.  The expressions
> from all `begin`, `include` and `include-ci`
> library declarations are expanded in that environment in the order in which
> they occur in the library.
> Alternatively, `cond-expand` and `import` declarations may be processed
> in left to right order interspersed with the processing of other
> declarations, with the environment growing as imported bindings are
> added to it by each `import` declaration.
> 
> When a library is loaded, its expressions are executed
> in textual order.
> If a library's definitions are referenced in the expanded form of a
> program or library body, then that library must be loaded before the
> expanded program or library body is evaluated. This rule applies
> transitively.  If a library is imported by more than one program or
> library, it may possibly be loaded additional times.
> 
> Similarly, during the expansion of a library `(foo)`, if any syntax
> keywords imported from another library `(bar)` are needed to expand
> the library, then the library `(bar)` must be expanded and its syntax
> definitions evaluated before the expansion of `(foo)`.
> 
> Regardless of the number of times that a library is loaded, each
> program or library that imports bindings from a library must do so from a
> single loading of that library, regardless of the number of import
> declarations in which it appears.
> That is, `(import (only (foo) a))` followed by `(import (only (foo) b))`
> has the same effect as `(import (only (foo) a b))`.
> 
> 
> ###### Library example
> 
> The following example shows
> how a program can be divided into libraries plus a relatively small
> main program (based on the paper __The fantastic combinations of John Conway's new solitaire game "Life"__).
> If the main program is entered into a REPL, it is not necessary to import
> the base library.
> 
> ````
> (define-library (example grid)
>   (export make rows cols ref each
>           (rename put! set!))
>   (import (scheme base))
>   (begin
>     ;; Create an NxM grid.
>     (define (make n m)
>       (let ((grid (make-vector n)))
>         (do ((i 0 (+ i 1)))
>             ((= i n) grid)
>           (let ((v (make-vector m #f)))
>             (vector-set! grid i v)))))
>     (define (rows grid)
>       (vector-length grid))
>     (define (cols grid)
>       (vector-length (vector-ref grid 0)))
>     ;; Return `#f` if out of range.
>     (define (ref grid n m)
>       (and (< -1 n (rows grid))
>            (< -1 m (cols grid))
>            (vector-ref (vector-ref grid n) m)))
>     (define (put! grid n m v)
>       (vector-set! (vector-ref grid n) m v))
>     (define (each grid proc)
>       (do ((j 0 (+ j 1)))
>           ((= j (rows grid)))
>         (do ((k 0 (+ k 1)))
>             ((= k (cols grid)))
>           (proc j k (ref grid j k)))))))
> 
> (define-library (example life)
>   (export life)
>   (import (except (scheme base) set!)
>           (scheme write)
>           (example grid))
>   (begin
>     (define (life-count grid i j)
>       (define (count i j)
>         (if (ref grid i j) 1 0))
>       (+ (count (- i 1) (- j 1))
>          (count (- i 1) j)
>          (count (- i 1) (+ j 1))
>          (count i (- j 1))
>          (count i (+ j 1))
>          (count (+ i 1) (- j 1))
>          (count (+ i 1) j)
>          (count (+ i 1) (+ j 1))))
>     (define (life-alive? grid i j)
>       (case (life-count grid i j)
>         ((3) #f)
>         ((2) (ref grid i j))
>         (else #f)))
>     (define (life-print grid)
>       (display "\x1B;[1H\x1B;[J")  ; clear vt100
>       (each grid
>        (lambda (i j v)
>          (display (if v "*" " "))
>          (when (= j (- (cols grid) 1))
>            (newline)))))
>     (define (life grid iterations)
>       (do ((i 0 (+ i 1))
>            (grid0 grid grid1)
>            (grid1 (make (rows grid) (cols grid))
>                   grid0))
>           ((= i iterations))
>         (each grid0
>          (lambda (j k v)
>            (let ((a (life-alive? grid0 j k)))
>              (set! grid1 j k a))))
>         (life-print grid1)))))
> 
> ;; Main program.
> (import (scheme base)
>         (only (example life) life)
>         (rename (prefix (example grid) grid-)
>                 (grid-make make-grid)))
> 
> ;; Initialize a grid with a glider.
> (define grid (make-grid 24 24))
> (grid-set! grid 1 1 #t)
> (grid-set! grid 2 2 #t)
> (grid-set! grid 3 0 #t)
> (grid-set! grid 3 1 #t)
> (grid-set! grid 3 2 #t)
> 
> ;; Run for 80 iterations.
> (life grid 80)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

