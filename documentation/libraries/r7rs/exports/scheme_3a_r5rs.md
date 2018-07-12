

<a id='export__r7rs__scheme_3a_r5rs'></a>

# `scheme:r5rs` -- `r7rs` Exports


<a id='export__r7rs__scheme_3a_r5rs__descriptor'></a>

#### Descriptor

````
(scheme r5rs)
````


<a id='export__r7rs__scheme_3a_r5rs__definitions'></a>

#### Definitions

 * [`interaction-environment`](../../r7rs/definitions/interaction-environment.md#definition__r7rs__interaction-environment);
 * [`scheme-report-environment`](../../r7rs/definitions/scheme-report-environment.md#definition__r7rs__scheme-report-environment);
 * [`null-environment`](../../r7rs/definitions/null-environment.md#definition__r7rs__null-environment);


<a id='export__r7rs__scheme_3a_r5rs__description'></a>

#### Description

> ##### R5RS Library
> 
> The `(scheme r5rs)` library provides the identifiers defined by
> __R5RS__, except that
> `transcript-on` and `transcript-off` are not present.
> Note that
> the `exact` and `inexact` procedures appear under their __R5RS__ names
> `inexact->exact` and `exact->inexact` respectively.
> However, if an implementation does not provide a particular library such as the
> complex library, the corresponding identifiers will not appear in this
> library either.
> 
> ````
> *                       +
> -                       /
> <                       <=
> =                       >
> >=                      abs
> acos                    and
> angle                   append
> apply                   asin
> assoc                   assq
> assv                    atan
> begin                   boolean?
> caaaar                  caaadr
> caaar                   caadar
> caaddr                  caadr
> caar                    cadaar
> cadadr                  cadar
> caddar                  cadddr
> caddr                   cadr
> call-with-current-continuation
> call-with-input-file    call-with-output-file
> call-with-values        car
> case                    cdaaar
> cdaadr                  cdaar
> cdadar                  cdaddr
> cdadr                   cdar
> cddaar                  cddadr
> cddar                   cdddar
> cddddr                  cdddr
> cddr                    cdr
> ceiling                 char->integer
> char-alphabetic?        char-ci<=?
> char-ci<?               char-ci=?
> char-ci>=?              char-ci>?
> char-downcase           char-lower-case?
> char-numeric?           char-ready?
> char-upcase             char-upper-case?
> char-whitespace?        char<=?
> char<?                  char=?
> char>=?                 char>?
> char?                   close-input-port
> close-output-port       complex?
> cond                    cons
> cos                     current-input-port
> current-output-port     define
> define-syntax           delay
> denominator             display
> do                      dynamic-wind
> eof-object?             eq?
> equal?                  eqv?
> eval                    even?
> exact->inexact          exact?
> exp                     expt
> floor                   for-each
> force                   gcd
> if                      imag-part
> inexact->exact          inexact?
> input-port?             integer->char
> integer?                interaction-environment
> lambda                  lcm
> length                  let
> let*                    let-syntax
> letrec                  letrec-syntax
> list                    list->string
> list->vector            list-ref
> list-tail               list?
> load                    log
> magnitude               make-polar
> make-rectangular        make-string
> make-vector             map
> max                     member
> memq                    memv
> min                     modulo
> negative?               newline
> not                     null-environment
> null?                   number->string
> number?                 numerator
> odd?                    open-input-file
> open-output-file        or
> output-port?            pair?
> peek-char               positive?
> procedure?              quasiquote
> quote                   quotient
> rational?               rationalize
> read                    read-char
> real-part               real?
> remainder               reverse
> round
> scheme-report-environment
> set!                    set-car!
> set-cdr!                sin
> sqrt                    string
> string->list            string->number
> string->symbol          string-append
> string-ci<=?            string-ci<?
> string-ci=?             string-ci>=?
> string-ci>?             string-copy
> string-fill!            string-length
> string-ref              string-set!
> string<=?               string<?
> string=?                string>=?
> string>?                string?
> substring               symbol->string
> symbol?                 tan
> truncate                values
> vector                  vector->list
> vector-fill!            vector-length
> vector-ref              vector-set!
> vector?                 with-input-from-file
> with-output-to-file     write
> write-char              zero?
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='export__r7rs__scheme_3a_r5rs__super-exports'></a>

#### Super-exports

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='export__r7rs__scheme_3a_r5rs__categories'></a>

#### Categories

 * [`r7rs:libraries`](../../r7rs/categories/r7rs_3a_libraries.md#category__r7rs__r7rs_3a_libraries);


<a id='export__r7rs__scheme_3a_r5rs__categories-recursive'></a>

#### Categories recursive

 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

