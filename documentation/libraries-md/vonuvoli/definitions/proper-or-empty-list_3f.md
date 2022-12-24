

<a id='definition__vonuvoli__proper-or-empty-list_3f'></a>

# `proper-or-empty-list?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__proper-or-empty-list_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__vonuvoli__proper-or-empty-list_3f__extends'></a>

#### Extends

 * [`list?`](../../r7rs/definitions/list_3f.md#definition__r7rs__list_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__proper-or-empty-list_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((null |1...|) -> (true))`
   * inputs:
     * a value of type [`null`](../../r7rs/types/null.md#type__r7rs__null);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((list-proper |1...|) -> (true))`
   * inputs:
     * a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((list-dotted |1...|) -> (false))`
   * inputs:
     * a value of type [`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((list-circular |1...|) -> (false))`
   * inputs:
     * a value of type [`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any |1...|) -> (false))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__proper-or-empty-list_3f__exports'></a>

#### Exports

 * [`vs:lists`](../../vonuvoli/exports/vs_3a_lists.md#export__vonuvoli__vs_3a_lists) -- `(vonuvoli pairs)`;


<a id='definition__vonuvoli__proper-or-empty-list_3f__exports-recursive'></a>

#### Exports recursive

 * [`vs:base`](../../vonuvoli/exports/vs_3a_base.md#export__vonuvoli__vs_3a_base) -- `(vonuvoli base)`;


<a id='definition__vonuvoli__proper-or-empty-list_3f__aliases'></a>

#### Aliases

 * `list?`;


<a id='definition__vonuvoli__proper-or-empty-list_3f__referenced-types'></a>

#### Referenced-types

 * [`null`](../../r7rs/types/null.md#type__r7rs__null);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);
 * [`list-dotted`](../../r7rs/types/list-dotted.md#type__r7rs__list-dotted);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`list-circular`](../../r7rs/types/list-circular.md#type__r7rs__list-circular);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__vonuvoli__proper-or-empty-list_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:lists`](../../vonuvoli/categories/vs_3a_lists.md#category__vonuvoli__vs_3a_lists);
 * [`vs:types`](../../vonuvoli/categories/vs_3a_types.md#category__vonuvoli__vs_3a_types);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

