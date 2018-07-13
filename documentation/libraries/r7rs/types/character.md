

<a id='type__r7rs__character'></a>

# `character` -- `r7rs` Types


<a id='type__r7rs__character__sub-types-tree'></a>

#### Sub-types tree

* **[`character-alphabetic`](../../r7rs/types/character-alphabetic.md#type__r7rs__character-alphabetic)**:
  * **[`character-alphabetic-upper-case`](../../r7rs/types/character-alphabetic-upper-case.md#type__r7rs__character-alphabetic-upper-case)**:
    * **[`character-ascii-alphabetic-upper-case`](../../r7rs/types/character-ascii-alphabetic-upper-case.md#type__r7rs__character-ascii-alphabetic-upper-case)**;
  * **[`character-alphabetic-lower-case`](../../r7rs/types/character-alphabetic-lower-case.md#type__r7rs__character-alphabetic-lower-case)**:
    * **[`character-ascii-alphabetic-lower-case`](../../r7rs/types/character-ascii-alphabetic-lower-case.md#type__r7rs__character-ascii-alphabetic-lower-case)**;
  * **[`character-ascii-alphabetic`](../../r7rs/types/character-ascii-alphabetic.md#type__r7rs__character-ascii-alphabetic)**:
    * [`character-ascii-alphabetic-upper-case`](../../r7rs/types/character-ascii-alphabetic-upper-case.md#type__r7rs__character-ascii-alphabetic-upper-case);
    * [`character-ascii-alphabetic-lower-case`](../../r7rs/types/character-ascii-alphabetic-lower-case.md#type__r7rs__character-ascii-alphabetic-lower-case);
* **[`character-numeric`](../../r7rs/types/character-numeric.md#type__r7rs__character-numeric)**:
  * **[`character-ascii-numeric`](../../r7rs/types/character-ascii-numeric.md#type__r7rs__character-ascii-numeric)**;
* **[`character-whitespace`](../../r7rs/types/character-whitespace.md#type__r7rs__character-whitespace)**:
  * **[`character-ascii-whitespace`](../../r7rs/types/character-ascii-whitespace.md#type__r7rs__character-ascii-whitespace)**;
* **[`character-ascii`](../../r7rs/types/character-ascii.md#type__r7rs__character-ascii)**:
  * [`character-ascii-alphabetic`](../../r7rs/types/character-ascii-alphabetic.md#type__r7rs__character-ascii-alphabetic):
    * [`character-ascii-alphabetic-upper-case`](../../r7rs/types/character-ascii-alphabetic-upper-case.md#type__r7rs__character-ascii-alphabetic-upper-case);
    * [`character-ascii-alphabetic-lower-case`](../../r7rs/types/character-ascii-alphabetic-lower-case.md#type__r7rs__character-ascii-alphabetic-lower-case);
  * [`character-ascii-numeric`](../../r7rs/types/character-ascii-numeric.md#type__r7rs__character-ascii-numeric);
  * [`character-ascii-whitespace`](../../r7rs/types/character-ascii-whitespace.md#type__r7rs__character-ascii-whitespace);


<a id='type__r7rs__character__super-types'></a>

#### Super-types

 * [(none)](../../r7rs/types/_index.md#toc__r7rs__types);


<a id='type__r7rs__character__sub-types'></a>

#### Sub-types

 * [`character-alphabetic`](../../r7rs/types/character-alphabetic.md#type__r7rs__character-alphabetic);
 * [`character-numeric`](../../r7rs/types/character-numeric.md#type__r7rs__character-numeric);
 * [`character-whitespace`](../../r7rs/types/character-whitespace.md#type__r7rs__character-whitespace);
 * [`character-ascii`](../../r7rs/types/character-ascii.md#type__r7rs__character-ascii);


<a id='type__r7rs__character__sub-types-recursive'></a>

##### Sub-types recursive

 * [`character-alphabetic-upper-case`](../../r7rs/types/character-alphabetic-upper-case.md#type__r7rs__character-alphabetic-upper-case);
 * [`character-alphabetic-lower-case`](../../r7rs/types/character-alphabetic-lower-case.md#type__r7rs__character-alphabetic-lower-case);
 * [`character-ascii-alphabetic`](../../r7rs/types/character-ascii-alphabetic.md#type__r7rs__character-ascii-alphabetic);
 * [`character-ascii-alphabetic-upper-case`](../../r7rs/types/character-ascii-alphabetic-upper-case.md#type__r7rs__character-ascii-alphabetic-upper-case);
 * [`character-ascii-alphabetic-lower-case`](../../r7rs/types/character-ascii-alphabetic-lower-case.md#type__r7rs__character-ascii-alphabetic-lower-case);
 * [`character-ascii-numeric`](../../r7rs/types/character-ascii-numeric.md#type__r7rs__character-ascii-numeric);
 * [`character-ascii-whitespace`](../../r7rs/types/character-ascii-whitespace.md#type__r7rs__character-ascii-whitespace);


<a id='type__r7rs__character__referent-definitions-input'></a>

#### Referent definitions as input

 * [`string`](../../r7rs/definitions/string.md#definition__r7rs__string);
 * [`make-string`](../../r7rs/definitions/make-string.md#definition__r7rs__make-string);
 * [`string-fill!`](../../r7rs/definitions/string-fill_21.md#definition__r7rs__string-fill_21);
 * [`string-set!`](../../r7rs/definitions/string-set_21.md#definition__r7rs__string-set_21);
 * [`write-char`](../../r7rs/definitions/write-char.md#definition__r7rs__write-char);
 * [`char?`](../../r7rs/definitions/char_3f.md#definition__r7rs__char_3f);
 * [`char=?`](../../r7rs/definitions/char_3d_3f.md#definition__r7rs__char_3d_3f);
 * [`char<?`](../../r7rs/definitions/char_3c_3f.md#definition__r7rs__char_3c_3f);
 * [`char>?`](../../r7rs/definitions/char_3e_3f.md#definition__r7rs__char_3e_3f);
 * [`char<=?`](../../r7rs/definitions/char_3c_3d_3f.md#definition__r7rs__char_3c_3d_3f);
 * [`char>=?`](../../r7rs/definitions/char_3e_3d_3f.md#definition__r7rs__char_3e_3d_3f);
 * [`char-ci=?`](../../r7rs/definitions/char-ci_3d_3f.md#definition__r7rs__char-ci_3d_3f);
 * [`char-ci<?`](../../r7rs/definitions/char-ci_3c_3f.md#definition__r7rs__char-ci_3c_3f);
 * [`char-ci>?`](../../r7rs/definitions/char-ci_3e_3f.md#definition__r7rs__char-ci_3e_3f);
 * [`char-ci<=?`](../../r7rs/definitions/char-ci_3c_3d_3f.md#definition__r7rs__char-ci_3c_3d_3f);
 * [`char-ci>=?`](../../r7rs/definitions/char-ci_3e_3d_3f.md#definition__r7rs__char-ci_3e_3d_3f);
 * [`char->integer`](../../r7rs/definitions/char-_3e_integer.md#definition__r7rs__char-_3e_integer);
 * [`digit-value`](../../r7rs/definitions/digit-value.md#definition__r7rs__digit-value);
 * [`char-alphabetic?`](../../r7rs/definitions/char-alphabetic_3f.md#definition__r7rs__char-alphabetic_3f);
 * [`char-upper-case?`](../../r7rs/definitions/char-upper-case_3f.md#definition__r7rs__char-upper-case_3f);
 * [`char-lower-case?`](../../r7rs/definitions/char-lower-case_3f.md#definition__r7rs__char-lower-case_3f);
 * [`char-numeric?`](../../r7rs/definitions/char-numeric_3f.md#definition__r7rs__char-numeric_3f);
 * [`char-whitespace?`](../../r7rs/definitions/char-whitespace_3f.md#definition__r7rs__char-whitespace_3f);
 * [`char-upcase`](../../r7rs/definitions/char-upcase.md#definition__r7rs__char-upcase);
 * [`char-downcase`](../../r7rs/definitions/char-downcase.md#definition__r7rs__char-downcase);
 * [`char-foldcase`](../../r7rs/definitions/char-foldcase.md#definition__r7rs__char-foldcase);


<a id='type__r7rs__character__referent-definitions-output'></a>

#### Referent definitions as output

 * [`string-ref`](../../r7rs/definitions/string-ref.md#definition__r7rs__string-ref);
 * [`string-set!`](../../r7rs/definitions/string-set_21.md#definition__r7rs__string-set_21);
 * [`integer->char`](../../r7rs/definitions/integer-_3e_char.md#definition__r7rs__integer-_3e_char);
 * [`char-upcase`](../../r7rs/definitions/char-upcase.md#definition__r7rs__char-upcase);
 * [`char-downcase`](../../r7rs/definitions/char-downcase.md#definition__r7rs__char-downcase);
 * [`char-foldcase`](../../r7rs/definitions/char-foldcase.md#definition__r7rs__char-foldcase);


<a id='type__r7rs__character__predicate'></a>

#### Predicate

````
char?
````


<a id='type__r7rs__character__description'></a>

#### Description

> Characters are objects that represent printed characters such as
> letters and digits.
> All Scheme implementations must support at least the ASCII character
> repertoire: that is, Unicode characters `U+0000` through `U+007F`.
> Implementations may support any other Unicode characters they see fit,
> and may also support non-Unicode characters as well.
> Except as otherwise specified, the result of applying any of the
> following procedures to a non-Unicode character is implementation-dependent.
> 
> Characters are written using the notation `#\<character>`
> or `#\<character name>` or
> `#\x<hex scalar value>`.
> 
> The following character names must be supported
> by all implementations with the given values.
> Implementations may add other names
> provided they cannot be interpreted as hex scalar values preceded by `x`.
> 
>   * `#\alarm` -- `U+0007`;
>   * `#\backspace` -- `U+0008`;
>   * `#\delete` -- `U+007F`;
>   * `#\escape` -- `U+001B`;
>   * `#\newline` -- the linefeed character, `U+000A`;
>   * `#\null` -- the null character, `U+0000`;
>   * `#\return` -- the return character, `U+000D`;
>   * `#\space` -- the preferred way to write a space;
>   * `#\tab` -- the tab character, `U+0009`;
> 
> Here are some additional examples:
> 
>   * `#\a` -- lower case letter;
>   * `#\A` -- upper case letter;
>   * `#\(` -- left parenthesis;
>   * `#\ ` (note the space after `\`) -- the space character;
>   * `#\x03BB` -- the `λ` character (if character is supported);
>   * `#\iota` -- the `ι` character (if character and name are supported);
> 
> Case is significant in `#\<character>`, and in
> `#\<character name>`,
> but not in `#\x<hex scalar value>`.
> If `<character>` in
> `#\<character>` is alphabetic, then any character
> immediately following `<character>` cannot be one that can appear in an identifier.
> This rule resolves the ambiguous case where, for
> example, the sequence of characters `#\space`
> could be taken to be either a representation of the space character or a
> representation of the character `#\s` followed
> by a representation of the symbol `pace`.
> 
> Characters written in the `#\` notation are self-evaluating.
> That is, they do not have to be quoted in programs.
> 
> Some of the procedures that operate on characters ignore the
> difference between upper case and lower case.  The procedures that
> ignore case have `-ci` (for __case insensitive__) embedded in their names.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__character__categories'></a>

#### Categories

 * [`r7rs:types-disjoint`](../../r7rs/categories/r7rs_3a_types-disjoint.md#category__r7rs__r7rs_3a_types-disjoint);
 * [`r7rs:types-characters`](../../r7rs/categories/r7rs_3a_types-characters.md#category__r7rs__r7rs_3a_types-characters);


<a id='type__r7rs__character__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

