

<a id='appendix__r7rs__language-changes'></a>

# Language changes


<a id='appendix__r7rs__language-changes__description'></a>

#### Description

> ##### Incompatibilities with __R5RS__
> 
> This section enumerates the incompatibilities between this report and
> the "Revised-5th report" [[R5RS]](#errors).
> 
> **Note**: This list is not authoritative, but is believed to be correct and complete.
> 
> * Case sensitivity is now the default in symbols and character names.
> This means that code written under the assumption that symbols could be
> written `FOO` or `Foo` in some contexts and `foo` in other contexts
> can either be changed, be marked with the new `#!fold-case` directive,
> or be included in a library using the `include-ci` library declaration.
> All standard identifiers are entirely in lower case.
> 
> * The `syntax-rules` construct now recognizes `_` (underscore)
> as a wildcard, which means it cannot be used as a syntax variable.
> It can still be used as a literal.
> 
> * The __R5RS__ procedures `exact->inexact` and `inexact->exact`
> have been renamed to their __R6RS__ names, `inexact` and `exact`,
> respectively, as these names are shorter and more correct.
> The former names are still available in the __R5RS__ library.
> 
> * The guarantee that string comparison (with `string<?` and the
> related predicates) is a lexicographical extension of character comparison
> (with `char<?` and the related predicates) has been removed.
> 
> * Support for the `#` character in numeric literals is no longer required.
> 
> * Support for the letters `s`, `f`, `d`, and `l`
> as exponent markers is no longer required.
> 
> * Implementations of `string->number` are no longer permitted
> to return `#f` when the argument contains an explicit radix prefix,
> and must be compatible with `read` and the syntax of numbers in programs.
> 
> * The procedures `transcript-on` and `transcript-off` have been removed.
> 
> 
> ##### Other language changes since __R5RS__
> 
> This section enumerates the additional differences between this report and
> the "Revised-5th report" [[R5RS]](#errors).
> 
> **Note**: This list is not authoritative, but is believed to be correct and complete.
> 
> * Various minor ambiguities and unclarities in __R5RS__ have been cleaned up.
> 
> * Libraries have been added as a new program structure to improve
> encapsulation and sharing of code.  Some existing and new identifiers
> have been factored out into separate libraries.
> Libraries can be imported into other libraries or main programs, with
> controlled exposure and renaming of identifiers.
> The contents of a library can be made conditional on the features of
> the implementation on which it is to be used.
> There is an __R5RS__ compatibility library.
> 
> * The expressions types `include`, `include-ci`, and `cond-expand`
> have been added to the base library; they have the same semantics as the
> corresponding library declarations.
> 
> * Exceptions can now be signaled explicitly with `raise`,
> `raise-continuable` or `error`, and can be handled with
> `with-exception-handler` and the `guard` syntax.
> Any object can specify an error condition; the implementation-defined
> conditions signaled by `error` have a predicate to detect them and accessor functions to
> retrieve the arguments passed to `error`.
> Conditions signaled by `read` and by file-related procedures
> also have predicates to detect them.
> 
> * New disjoint types supporting access to multiple fields can be
> generated with the `define-record-type` of SRFI 9 [[SRFI-9]](#errors)
> 
> * Parameter objects can be created with `make-parameter`, and
> dynamically rebound with `parameterize`.
> The procedures `current-input-port` and `current-output-port` are now
> parameter objects, as is the newly introduced `current-error-port`.
> 
> * Support for promises has been enhanced based on SRFI 45 [[SRFI-45]](#errors).
> 
> * __Bytevectors__, vectors of exact integers in the range
> from 0 to 255 inclusive, have been added as a new disjoint type.
> A subset of the vector procedures is provided.  Bytevectors
> can be converted to and from strings in accordance with the UTF-8 character encoding.
> Bytevectors have a datum representation and evaluate to themselves.
> 
> * Vector constants evaluate to themselves.
> 
> * The procedure `read-line` is provided to make line-oriented textual input
> simpler.
> 
> * The procedure `flush-output-port` is provided to allow minimal
> control of output port buffering.
> 
> * __Ports__ can now be designated as __textual__ or
> __binary__ ports, with new procedures for reading and writing binary
> data.
> The new predicates `input-port-open?` and `output-port-open?` return whether a port is open or closed.
> The new procedure `close-port` now closes a port; if the port
> has both input and output sides, both are closed.
> 
> * __String ports__ have been added as a way to read and write
> characters to and from strings, and __bytevector ports__ to read
> and write bytes to and from bytevectors.
> 
> * There are now I/O procedures specific to strings and bytevectors.
> 
> * The `write` procedure now generates datum labels when applied to
> circular objects.  The new procedure `write-simple` never generates
> labels; `write-shared` generates labels for all shared and circular
> structure.
> The `display` procedure must not loop on circular objects.
> 
> * The __R6RS__ procedure `eof-object` has been added.
> Eof-objects are now required to be a disjoint type.
> 
> * Syntax definitions are now allowed wherever variable definitions are.
> 
> * The `syntax-rules` construct now allows
> the ellipsis symbol to be specified explicitly instead of the default
> `...`, allows template escapes with an ellipsis-prefixed list, and
> allows tail patterns to follow an ellipsis pattern.
> 
> * The `syntax-error` syntax has been added as a way to signal immediate
> and more informative errors when a macro is expanded.
> 
> * The `letrec*` binding construct has been added, and internal `define`
> is specified in terms of it.
> 
> * Support for capturing multiple values has been enhanced with
> `define-values`, `let-values`, and `let*-values`.
> Standard expression types which contain a sequence of expressions now
> permit passing zero or more than one value to the continuations of all
> non-final expressions of the sequence.
> 
> * The `case` conditional now supports `=>` syntax
> analogous to `cond` not only in regular clauses but in the
> `else` clause as well.
> 
> * To support dispatching on the number of arguments passed to a
> procedure, `case-lambda` has been added in its own library.
> 
> * The convenience conditionals `when` and `unless` have been added.
> 
> * The behavior of `eqv?` on inexact numbers now conforms to the
> __R6RS__ definition.
> 
> * When applied to procedures, `eq?` and `eqv?` are permitted to
> return different answers.
> 
> * The __R6RS__ procedures `boolean=?` and `symbol=?` have been added.
> 
> * Positive infinity, negative infinity, `NaN`, and negative inexact zero have been added
> to the numeric tower as inexact values with the written
> representations `+inf.0`, `-inf.0`, `+nan.0`, and `-0.0`
> respectively.  Support for them is not required.
> The representation `-nan.0` is synonymous with `+nan.0`.
> 
> * The `log` procedure now accepts a second argument specifying
> the logarithm base.
> 
> * The procedures `map` and `for-each` are now required to terminate on
> the shortest argument list.
> 
> * The procedures `member` and `assoc` now take an optional third argument
> specifying the equality predicate to be used.
> 
> * The numeric procedures `finite?`, `infinite?`, `nan?`,
> `exact-integer?`, `square`, and `exact-integer-sqrt`
> have been added.
> 
> * The `-` and `/` procedures
> and the character and string comparison
> predicates are now required to support more than two arguments.
> 
> * The forms `#t` and `#f` are now supported
> as well as `#t` and `#f`.
> 
> * The procedures `make-list`, `list-copy`, `list-set!`,
> `string-map`, `string-for-each`, `string->vector`,
> `vector-append`,
> `vector-copy`, `vector-map`, `vector-for-each`,
> `vector->string`, `vector-copy!`, and `string-copy!`
> have been added to round out the sequence operations.
> 
> * Some string and vector procedures support processing of part of a string or vector using
> optional `start` and `end` arguments.
> 
> * Some list procedures are now defined on circular lists.
> 
> * Implementations may provide any subset of the full Unicode
> repertoire that includes ASCII, but implementations must support any
> such subset in a way consistent with Unicode.
> Various character and string procedures have been extended accordingly,
> and case conversion procedures added for strings.
> String comparison is no longer
> required to be consistent with character comparison, which is based
> solely on Unicode scalar values.
> The new `digit-value` procedure has been added to obtain the numerical
> value of a numeric character.
> 
> * There are now two additional comment syntaxes: `#;` to
> skip the next datum, and `#| ... |#`
> for nestable block comments.
> 
> * Data prefixed with datum labels `#<n>=` can be referenced
> with `#<n>#`, allowing for reading and writing of data with
> shared structure.
> 
> * Strings and symbols now allow mnemonic and numeric escape
> sequences, and the list of named characters has been extended.
> 
> * The procedures `file-exists?` and `delete-file` are available in the
> `(scheme file)` library.
> 
> * An interface to the system environment, command line, and process exit status is
> available in the `(scheme process-context)` library.
> 
> * Procedures for accessing time-related values are available in the
> `(scheme time)` library.
> 
> * A less irregular set of integer division operators is provided
> with new and clearer names.
> 
> * The `load` procedure now accepts a second argument specifying the environment to
> load into.
> 
> * The `call-with-current-continuation` procedure now has the synonym
> `call/cc`.
> 
> * The semantics of read-eval-print loops are now partly prescribed,
> requiring the redefinition of procedures, but not syntax keywords, to have retroactive effect.
> 
> * The formal semantics now handles `dynamic-wind`.
> 
> 
> ##### Incompatibilities with __R6RS__
> 
> This section enumerates the incompatibilities between __R7RS__ and
> the "Revised-6th report" [[R6RS]](#errors) and its accompanying Standard Libraries document.
> 
> **Note**: This list is not authoritative, and is possibly incomplete.
> 
> * __R7RS__ libraries begin with the keyword `define-library`
> rather than `library` in order to make them syntactically
> distinguishable from __R6RS__ libraries.
> In __R7RS__ terms, the body of an __R6RS__ library consists
> of a single export declaration followed by a single import declaration,
> followed by commands and definitions.  In __R7RS__, commands and
> definitions are not permitted directly within the body: they have to be be wrapped in a `begin`
> library declaration.
> 
> * There is no direct __R6RS__ equivalent of the `include`, `include-ci`,
> `include-library-declarations`, or `cond-expand` library declarations.
> On the other hand, the __R7RS__ library syntax does not support phase or version specifications.
> 
> * The grouping of standardized identifiers into libraries is different from the __R6RS__
> approach. In particular, procedures which are optional in __R5RS__, either expressly
> or by implication, have been removed from the base library.
> Only the base library itself is an absolute requirement.
> 
> * No form of identifier syntax is provided.
> 
> * Internal syntax definitions are allowed, but uses of a syntax form
> cannot appear before its definition; the `even`/`odd` example given in
> __R6RS__ is not allowed.
> 
> * The __R6RS__ exception system was incorporated as-is, but the condition
> types have been left unspecified.  In particular, where __R6RS__ requires
> a condition of a specified type to be signaled, __R7RS__ says only
> "it is an error", leaving the question of signaling open.
> 
> * Full Unicode support is not required.
> Normalization is not provided.
> Character comparisons are
> defined by Unicode, but string comparisons are implementation-dependent.
> Non-Unicode characters are permitted.
> 
> * The full numeric tower is optional as in __R5RS__, but optional support for IEEE
> infinities, `NaN`, and `-0.0` was adopted from __R6RS__. Most clarifications on
> numeric results were also adopted, but the __R6RS__ procedures `real-valued?`,
> `rational-valued?`, and `integer-valued`? were not.
> The __R6RS__ division operators `div`, `mod`, `div-and-mod`,
> `div0`, `mod0` and `div0-and-mod0` are not provided.
> 
> * When a result is unspecified, it is still required to be a single value.
> However, non-final expressions
> in a body can return any number of values.
> 
> * The semantics of `map` and `for-each` have been changed to use
> the SRFI 1 [[SRFI-1]](#errors) early termination behavior. Likewise,
> `assoc` and `member` take an optional `equal?` argument as in SRFI 1,
> instead of the separate `assp` and `memp` procedures of __R6RS__.
> 
> * The __R6RS__ `quasiquote` clarifications have been adopted, with the
> exception of multiple-argument `unquote` and
> `unquote-splicing`.
> 
> * The __R6RS__ method of specifying mantissa widths was not adopted.
> 
> * String ports are compatible with SRFI 6 [[SRFI-6]](#errors) rather than __R6RS__.
> 
> * __R6RS__-style bytevectors are included, but
> only the unsigned byte (`u8`) procedures have been provided.
> The lexical syntax uses `#u8` for compatibility
> with SRFI 4 [[SRFI-4]](#errors), rather than the __R6RS__ `#vu8` style.
> 
> * The utility macros `when` and `unless` are provided, but
> their result is left unspecified.
> 
> * The remaining features of the Standard Libraries document were
> left to future standardization efforts.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

