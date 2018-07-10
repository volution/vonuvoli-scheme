

<a id='definition__r7rs__write-char'></a>

# `write-char` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `((|character|) |->| (|undefined|))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|character|) |->| (|void|))`
   * input: a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
   * requires: `|vonuvoli|`
 * `((|character| |textual-output-port-open|) |->| (|undefined|))`
   * inputs:
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
   * requires: `(|not| |vonuvoli|)`
 * `((|character| |textual-output-port-open|) |->| (|void|))`
   * inputs:
     * a value of type [`character`](../../r7rs/types/character.md#type__r7rs__character);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
   * requires: `|vonuvoli|`


#### Referenced types

[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
[`void`](../../r7rs/types/void.md#type__r7rs__void);
[`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);


#### Description

> ````
> (write-char char)
> (write-char char port)
> ````
> 
> 
> Writes the character `char` (not an external representation of the
> character) to the given textual output `port` and returns an unspecified
> value.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
[`vs:strings`](../../r7rs/categories/vs_3a_strings.md#category__r7rs__vs_3a_strings);
[`vs:characters`](../../r7rs/categories/vs_3a_characters.md#category__r7rs__vs_3a_characters);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

