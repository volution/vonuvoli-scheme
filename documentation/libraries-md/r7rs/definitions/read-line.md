

<a id='definition__r7rs__read-line'></a>

# `read-line` -- `r7rs` Definition


<a id='definition__r7rs__read-line__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__read-line__implemented-by'></a>

#### Implemented by

 * [`read-string-line`](../../vonuvoli/definitions/read-string-line.md#definition__vonuvoli__read-string-line) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__read-line__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (string-not-empty-or-eof))`
   * inputs: none;
   * output: a value of type [`string-not-empty-or-eof`](../../r7rs/types/string-not-empty-or-eof.md#type__r7rs__string-not-empty-or-eof);
 * `((textual-input-port-eof) -> (eof-object))`
   * input: a value of type [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((textual-input-port-open) -> (string-not-empty-or-eof))`
   * input: a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
   * output: a value of type [`string-not-empty-or-eof`](../../r7rs/types/string-not-empty-or-eof.md#type__r7rs__string-not-empty-or-eof);


<a id='definition__r7rs__read-line__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__read-line__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__read-line__description'></a>

#### Description

> ````
> (read-line)
> (read-line port)
> ````
> 
> 
> Returns the next line of text available from the textual input
> `port`, updating the `port` to point to the following character.
> If an end of line is read, a string containing all of the text up to
> (but not including) the end of line is returned, and the port is updated
> to point just past the end of line. If an end of file is encountered
> before any end of line is read, but some characters have been
> read, a string containing those characters is returned. If an end of
> file is encountered before any characters are read, an end-of-file
> object is returned.  For the purpose of this procedure, an end of line
> consists of either a linefeed character, a carriage return character, or a
> sequence of a carriage return character followed by a linefeed character.
> Implementations may also recognize other end of line characters or sequences.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__read-line__referenced-types'></a>

#### Referenced-types

 * [`string-not-empty-or-eof`](../../r7rs/types/string-not-empty-or-eof.md#type__r7rs__string-not-empty-or-eof);
 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
 * [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

