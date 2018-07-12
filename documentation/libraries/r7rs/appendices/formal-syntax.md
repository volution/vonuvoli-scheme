

<a id='appendix__r7rs__formal-syntax'></a>

# `r7rs` -- Formal syntax


<a id='appendix__r7rs__formal-syntax__description'></a>

#### Description

> This section provides a formal syntax for Scheme written in an extended
> BNF.
> 
> All spaces in the grammar are for legibility.  Case is not significant
> except in the definitions of `<letter>`, `<character name>` and `<mnemonic escape>`; for example, `#x1A`
> and `#X1a` are equivalent, but `foo` and `Foo`
> and `#\space` and `#\Space` are distinct.
> `<empty>` stands for the empty string.
> 
> The following extensions to BNF are used to make the description more
> concise:  `<thing>*` means zero or more occurrences of
> `<thing>`; and `<thing>+` means at least one
> `<thing>`.
> 
> 
> ##### Lexical structure
> 
> This section describes how individual tokens (identifiers,
> numbers, etc.) are formed from sequences of characters.  The following
> sections describe how expressions and programs are formed from sequences
> of tokens.
> 
> `<Intertoken space>` can occur on either side of any token, but not
> within a token.
> 
> Identifiers that do not begin with a vertical line are
> terminated by a `<delimiter>` or by the end of the input.
> So are dot, numbers, characters, and booleans.
> Identifiers that begin with a vertical line are terminated by another vertical line.
> 
> The following four characters from the __ASCII__ repertoire
> are reserved for future extensions to the
> language: `[`, `]`, `{`, `}`.
> 
> In addition to the identifier characters of the __ASCII__ repertoire specified
> below, Scheme implementations may permit any additional repertoire of
> Unicode characters to be employed in identifiers,
> provided that each such character has a Unicode general category of `Lu`,
> `Ll`, `Lt`, `Lm`, `Lo`, `Mn`, `Mc`, `Me`, `Nd`, `Nl`, `No`, `Pd`, `Pc`, `Po`, `Sc`, `Sm`, `Sk`, `So`,
> or `Co`, or is `U+200C` or `U+200D` (the zero-width non-joiner and joiner,
> respectively, which are needed for correct spelling in Persian, Hindi,
> and other languages).
> However, it is an error for the first character to have a general category
> of `Nd`, `Mc`, or `Me`.  It is also an error to use a non-Unicode character
> in symbols or identifiers.
> 
> All Scheme implementations must permit the escape sequence
> `\x<hexdigits>;`
> to appear in Scheme identifiers that are enclosed in vertical lines. If the character
> with the given Unicode scalar value is supported by the implementation,
> identifiers containing such a sequence are equivalent to identifiers
> containing the corresponding character.
> 
> ````
> <token> --->
>     | <identifier>
>     | <boolean>
>     | <number>
>     | <character>
>     | <string>
>     | `(`
>     | `)`
>     | `#(`
>     | `#u8(`
>     | `'`
>     | `$\backquote$`
>     | `,`
>     | `,@`
>     | `.`
> <delimiter> --->
>     | <whitespace>
>     | <vertical line>
>     | `(`
>     | `)`
>     | `"`
>     | `;`
> <intraline whitespace> --->
>     | <space or tab>
> <whitespace> --->
>     | <intraline whitespace>
>     | <line ending>
> <vertical line> --->
>     | `|`
> <line ending> --->
>     | <newline>
>     | <return> <newline>
>     | <return>
> <comment> --->
>     ; <all subsequent characters up to a line ending>
>     | <nested comment>
>     | `#;` <intertoken space> <datum>
> <nested comment> --->
>     | `#|` <comment text> <comment cont>* `|#`
> <comment text> --->
>     | <character sequence not containing `#|` or `|#`>
> <comment cont> --->
>     | <nested comment> <comment text>
> <directive> --->
>     | `#!fold-case`
>     | `#!no-fold-case`
> ````
> 
> Note that it is ungrammatical to follow a `<directive>` with anything
> but a `<delimiter>` or the end of file.
> 
> ````
> <atmosphere> --->
>     | <whitespace>
>     | <comment>
>     | <directive>
> <intertoken space> --->
>     | <atmosphere>*
> ````
> 
> Note that `+i`, `-i` and `<infnan>` below are exceptions to the
> `<peculiar identifier>` rule; they are parsed as numbers, not
> identifiers.
> 
> ````
> <identifier> --->
>     | <initial> <subsequent>*
>     | <vertical line> <symbol element>* <vertical line>
>     | <peculiar identifier>
> <initial> --->
>     | <letter>
>     | <special initial>
> <letter> --->
>     | `a` | `b` | `c` | ... | `z`
>     | `A` | `B` | `C` | ... | `Z`
> <special initial> --->
>     | `!` | `$` | `%` | `&` | `*`
>     | `/` | `:` | `<` | `=` | `>`
>     | `?` | `^` | `_` | `~`
> <subsequent> --->
>     | <initial>
>     | <digit>
>     | <special subsequent>
> <digit> --->
>     | `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9`
> <hex digit> --->
>     | <digit>
>     | `a` | `b` | `c` | `d` | `e` | `f`
> <explicit sign> --->
>     | `+`
>     | `-`
> <special subsequent> --->
>     | <explicit sign>
>     | `.`
>     | `@`
> <inline hex escape> --->
>     | `\x` <hex scalar value> `;`
> <hex scalar value> --->
>     | <hex digit>+
> <mnemonic escape> --->
>     | `\a`
>     | `\b`
>     | `\t`
>     | `\n`
>     | `\r`
> <peculiar identifier> --->
>     | <explicit sign>
>     | <explicit sign> <sign subsequent> <subsequent>*
>     | <explicit sign> `.` <dot subsequent> <subsequent>*
>     | `.` <dot subsequent> <subsequent>*
> <dot subsequent> --->
>     | <sign subsequent>
>     | `.`
> <sign subsequent> --->
>     | <initial>
>     | <explicit sign>
>     | `@`
> <symbol element> --->
>     | <any character other than <vertical line> or `\`>
>     | <inline hex escape>
>     | <mnemonic escape>
>     | `\|`
> 
> <boolean> --->
>     | `#t`
>     | `#f`
>     | `#true`
>     | `#false`
> <character> --->
>     | `#\` <any character>
>     | `#\` <character name>
>     | `#\x` <hex scalar value>
> <character name> --->
>     | `alarm`
>     | `backspace`
>     | `delete`
>     | `escape`
>     | `newline`
>     | `null`
>     | `return`
>     | `space`
>     | `tab`
> <string> --->
>     | `"` <string element>* `"`
> <string element> --->
>     | <any character other than `"` or `\`>
>     | <mnemonic escape>
>     | `\"`
>     | `\\`
>     | `\` <intraline whitespace>* <line ending> <intraline whitespace>*
>     | <inline hex escape>
> <bytevector> --->
>     | `#u8(` <byte>* `)`
> <byte> --->
>     | <any exact integer between `0` and `255`>
> ````
> 
> ````
> <number> --->
>     | <num 2>
>     | <num 8>
>     | <num 10>
>     | <num 16>
> ````
> 
> The following rules for `<num R>`, `<complex R>`,
> `<real R>`, `<ureal R>`, `<uinteger R>`, and `<prefix R>`
> are implicitly replicated for `R = 2, 8, 10, and 16`.
> There are no rules for `<decimal 2>`,
> `<decimal 8>`, and `<decimal 16>`, which means that numbers containing
> decimal points or exponents are always in decimal radix.
> Although not shown below, all alphabetic characters used in the grammar
> of numbers can appear in either upper or lower case.
> 
> ````
> <num R> --->
>     | <prefix R> <complex R>
> <complex R> --->
>     | <real R>
>     | <real R> `@` <real R>
>     | <real R> `+` <ureal R> `i`
>     | <real R> `-` <ureal R> `i`
>     | <real R> `+` `i`
>     | <real R> `-` `i`
>     | <real R> <infnan> `i`
>     | `+` <ureal R> `i`
>     | `-` <ureal R> `i`
>     | <infnan> `i`
>     | `+` `i`
>     | `-` `i`
> <real R> --->
>     | <sign> <ureal R>
>     | <infnan>
> <ureal R> --->
>     | <uinteger R>
>     | <uinteger R> `/` <uinteger R>
>     | <decimal R>
> <decimal 10> --->
>     | <uinteger 10> <suffix>
>     | `.` <digit 10>+ <suffix>
>     | <digit 10>+ `.` <digit 10>* <suffix>
> <uinteger R> --->
>     | <digit R>+
> <prefix R> --->
>     | <radix R> <exactness>
>     | <exactness> <radix R>
> <infnan> --->
>     | `+inf.0`
>     | `-inf.0`
>     | `+nan.0`
>     | `-nan.0`
> ````
> 
> ````
> <suffix> --->
>     | <empty>
>     | <exponent marker> <sign> <digit 10>+
> <exponent marker> --->
>     | `e`
> <sign> --->
>     | <empty>
>     | `+`
>     | `-`
> <exactness> --->
>     | <empty>
>     | `#i`
>     | `#e`
> <radix 2> --->
>     | `#b`
> <radix 8> --->
>     | `#o`
> <radix 10> --->
>     | <empty>
>     | `#d`
> <radix 16> --->
>     | `#x`
> <digit 2> --->
>     | `0` | `1`
> <digit 8> --->
>     | `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7`
> <digit 10> --->
>     | <digit>
> <digit 16> --->
>     | <digit 10>
>     | `a` | `b` | `c` | `d` | `e` | `f`
> ````
> 
> 
> ##### External representations
> 
> `<Datum>` is what the `read` procedure (section on `read`)
> successfully parses.  Note that any string that parses as an
> `<expression>` will also parse as a `<datum>`.
> 
> ````
> <datum> --->
>     | <simple datum>
>     | <compound datum>
>     | <label> `=` <datum>
>     | <label> `#`
> <simple datum> --->
>     | <boolean>
>     | <number>
>     | <character>
>     | <string>
>     |  <symbol>
>     | <bytevector>
> <symbol> --->
>     | <identifier>
> <compound datum> --->
>     | <list>
>     | <vector>
>     | <abbreviation>
> <list> --->
>     | `(` <datum>* `)`
>     | `(` <datum>+ `.` <datum> `)`
> <abbreviation> --->
>     | <abbrev prefix> <datum>
> <abbrev prefix> --->
>     | `'`
>     | `$\backquote$`
>     | `,`
>     | `,@`
> <vector> --->
>     | `#(` <datum>* `)`
> <label> --->
>     | `#` <uinteger 10>
> ````
> 
> 
> ##### Expressions
> 
> The definitions in this and the following subsections assume that all
> the syntax keywords defined in this report have been properly imported
> from their libraries, and that none of them have been redefined or shadowed.
> 
> ````
> <expression> --->
>     | <identifier>
>     | <literal>
>     | <procedure call>
>     | <lambda expression>
>     | <conditional>
>     | <assignment>
>     | <derived expression>
>     | <macro use>
>     | <macro block>
>     | <includer>
> 
> <literal> --->
>     | <quotation>
>     | <self-evaluating>
> <self-evaluating> --->
>     | <boolean>
>     | <number>
>     | <vector>
>     | <character>
>     | <string>
>     | <bytevector>
> <quotation> --->
>     | '<datum>
>     | (quote <datum>)
> <procedure call> --->
>     | (<operator> <operand>*)
> <operator> --->
>     | <expression>
> <operand> --->
>     | <expression>
> 
> <lambda expression> --->
>     | (lambda <formals> <body>)
> <formals> --->
>     | (<identifier>*)
>     | <identifier>
>     | (<identifier>+ . <identifier>)
> <body> --->
>     | <definition>* <sequence>
> <sequence> --->
>     | <command>* <expression>
> <command> --->
>     | <expression>
> 
> <conditional> --->
>     | (if <test> <consequent> <alternate>)
> <test> --->
>     | <expression>
> <consequent> --->
>     | <expression>
> <alternate> --->
>     | <expression>
>     | <empty>
> 
> <assignment> --->
>     | (set! <identifier> <expression>)
> 
> <derived expression> --->
>     | (cond <cond clause>+)
>     | (cond <cond clause>* (else <sequence>))
>     | (case <expression> <case clause>+)
>     | (case <expression> <case clause>* (else <sequence>))
>     | (case <expression> <case clause>* (else => <recipient>))
>     | (and <test>*)
>     | (or <test>*)
>     | (when <test> <sequence>)
>     | (unless <test> <sequence>)
>     | (let (<binding spec>*) <body>)
>     | (let <identifier> (<binding spec>*) <body>)
>     | (let* (<binding spec>*) <body>)
>     | (letrec (<binding spec>*) <body>)
>     | (letrec* (<binding spec>*) <body>)
>     | (let-values (<mv binding spec>*) <body>)
>     | (let*-values (<mv binding spec>*) <body>)
>     | (begin <sequence>)
>     | (do (<iteration spec>*) (<test> <do result>) <command>*)
>     | (delay <expression>)
>     | (delay-force <expression>)
>     | (parameterize ((<expression> <expression>)*) <body>)
>     | (guard (<identifier> <cond clause>*) <body>)
>     | <quasiquotation>
>     | (case-lambda <case-lambda clause>*)
> 
> <cond clause> --->
>     | (<test> <sequence>)
>     | (<test>)
>     | (<test> => <recipient>)
> <recipient> --->
>     | <expression>
> <case clause> --->
>     | ((<datum>*) <sequence>)
>     | ((<datum>*) => <recipient>)
> <binding spec> --->
>     | (<identifier> <expression>)
> <mv binding spec> --->
>     | (<formals> <expression>)
> <iteration spec> --->
>     | (<identifier> <init> <step>)
>     | (<identifier> <init>)
> <case-lambda clause> --->
>     | (<formals> <body>)
> <init> --->
>     | <expression>
> <step> --->
>     | <expression>
> <do result> --->
>     | <sequence>
>     | <empty>
> 
> <macro use> --->
>     | (<keyword> <datum>*)
> <keyword> --->
>     | <identifier>
> 
> <macro block> --->
>     | (let-syntax (<syntax spec>*) <body>)
>     | (letrec-syntax (<syntax spec>*) <body>)
> <syntax spec> --->
>     | (<keyword> <transformer spec>)
> 
> <includer> --->
>     | (include <string>+)
>     | (include-ci <string>+)
> ````
> 
> 
> ##### Quasiquotations
> 
> The following grammar for quasiquote expressions is not context-free.
> It is presented as a recipe for generating an infinite number of
> production rules.  Imagine a copy of the following rules for
> `D = 1, 2, 3, ...`, where `D` is the nesting depth.
> 
> ````
> <quasiquotation> --->
>     | <quasiquotation 1>
> <qq template 0> --->
>     | <expression>
> <quasiquotation D> --->
>     | `<qq template D>
>     | (quasiquote <qq template D>)
> <qq template D> --->
>     | <simple datum>
>     | <list qq template D>
>     | <vector qq template D>
>     | <unquotation D>
> <list qq template D> --->
>     | (<qq template or splice D>*)
>     | (<qq template or splice D>+ . <qq template D>)
>     | '<qq template D>
>     | <quasiquotation D+1>
> <vector qq template D> --->
>     | #(<qq template or splice D>*)
> <unquotation D> --->
>     | ,<qq template D-1>
>     | (unquote <qq template D-1>)
> <qq template or splice D> --->
>     | <qq template D>
>     | <splicing unquotation D>
> <splicing unquotation D> --->
>     | ,@<qq template D-1>
>     | (unquote-splicing <qq template D-1>)
> ````
> 
> In `<quasiquotation>`s, a `<list qq template D>` can sometimes
> be confused with either an `<unquotation D>` or a
> `<splicing unquotation D>`.  The interpretation as an
> `<unquotation>` or
> `<splicing unquotation D>` takes precedence.
> 
> 
> ##### Transformers
> 
> ````
> <transformer spec> --->
>     | (syntax-rules (<identifier>*) <syntax rule>*)
>     | (syntax-rules <identifier> (<identifier>*) <syntax rule>*)
> <syntax rule> --->
>     | (<pattern> <template>)
> <pattern> --->
>     | <pattern identifier>
>     | <underscore>
>     | (<pattern>*)
>     | (<pattern>+ . <pattern>)
>     | (<pattern>* <pattern> <ellipsis> <pattern>*)
>     | (<pattern>* <pattern> <ellipsis> <pattern>* . <pattern>)
>     | #(<pattern>*)
>     | #(<pattern>* <pattern> <ellipsis> <pattern>*)
>     | <pattern datum>
> <pattern datum> --->
>     | <string>
>     | <character>
>     | <boolean>
>     | <number>
> <template> --->
>     | <pattern identifier>
>     | (<template element>*)
>     | (<template element>+ . <template>)
>     | #(<template element>*)
>     | <template datum>
> <template element> --->
>     | <template>
>     | <template> <ellipsis>
> <template datum> --->
>     | <pattern datum>
> <pattern identifier> --->
>     | <any identifier except ...>
> <ellipsis> --->
>     | <an identifier defaulting to ...>
> <underscore> --->
>     | <the identifier _>
> ````
> 
> 
> ##### Programs and definitions
> 
> ````
> <program> --->
>     | <import declaration>+
>     | <command or definition>+
> <command or definition> --->
>     | <command>
>     | <definition>
>     | (begin <command or definition>+)
> <definition> --->
>     | (define <identifier> <expression>)
>     | (define (<identifier> <def formals>) <body>)
>     | <syntax definition>
>     | (define-values <formals> <body>)
>     | (define-record-type <identifier> <constructor> <identifier> <field spec>*)
>     | (begin <definition>*)
> <def formals> --->
>     | <identifier>*
>     | <identifier>* . <identifier>
> <constructor> --->
>     | (<identifier> <field name>*)
> <field spec> --->
>     | (<field name> <accessor>)
>     | (<field name> <accessor> <mutator>)
> <field name> --->
>     | <identifier>
> <accessor> --->
>     | <identifier>
> <mutator> --->
>     | <identifier>
> <syntax definition> --->
>     | (define-syntax <keyword> <transformer spec>)
> ````
> 
> 
> ##### Libraries
> 
> ````
> <library> --->
>     | (define-library <library name> <library declaration>*)
> <library name> --->
>     | (<library name part>+)
> <library name part> --->
>     | <identifier>
>     | <uinteger 10>
> <library declaration> --->
>     | (export <export spec>*)
>     | <import declaration>
>     | (begin <command or definition>*)
>     | <includer>
>     | (include-library-declarations <string>+)
>     | (cond-expand <cond-expand clause>+)
>     | (cond-expand <cond-expand clause>+ (else <library declaration>*))
> <import declaration> --->
>     | (import <import set>+)
> <export spec> --->
>     | <identifier>
>     | (rename <identifier> <identifier>)
> <import set> --->
>     | <library name>
>     | (only <import set> <identifier>+)
>     | (except <import set> <identifier>+)
>     | (prefix <import set> <identifier>)
>     | (rename <import set> (<identifier> <identifier>)+)
> <cond-expand clause> --->
>     (<feature requirement> <library declaration>*)
> <feature requirement> --->
>     | <identifier>
>     | <library name>
>     | (and <feature requirement>*)
>     | (or <feature requirement>*)
>     | (not <feature requirement>)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

