

<a id='definition__vonuvoli__input-port-open_3f'></a>

# `input-port-open?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__input-port-open_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__vonuvoli__input-port-open_3f__extends'></a>

#### Extends

 * [`input-port-open?`](../../r7rs/definitions/input-port-open_3f.md#definition__r7rs__input-port-open_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__input-port-open_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((input-port-open |1...|) -> (true))`
   * inputs:
     * a value of type [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((input-port |1...|) -> (false))`
   * inputs:
     * a value of type [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((port |1...|) -> (false))`
   * inputs:
     * a value of type [`port`](../../r7rs/types/port.md#type__r7rs__port);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any |1...|) -> (false))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__input-port-open_3f__exports'></a>

#### Exports

 * [`vs:io`](../../vonuvoli/exports/vs_3a_io.md#export__vonuvoli__vs_3a_io) -- `(vonuvoli io)`;


<a id='definition__vonuvoli__input-port-open_3f__referenced-types'></a>

#### Referenced-types

 * [`input-port-open`](../../r7rs/types/input-port-open.md#type__r7rs__input-port-open);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`input-port`](../../r7rs/types/input-port.md#type__r7rs__input-port);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__vonuvoli__input-port-open_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:ports:input`](../../vonuvoli/categories/vs_3a_ports_3a_input.md#category__vonuvoli__vs_3a_ports_3a_input);
 * [`vs:ports:open`](../../vonuvoli/categories/vs_3a_ports_3a_open.md#category__vonuvoli__vs_3a_ports_3a_open);


<a id='definition__vonuvoli__input-port-open_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../vonuvoli/categories/vs_3a_ports.md#category__vonuvoli__vs_3a_ports);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

