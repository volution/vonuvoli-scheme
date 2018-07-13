
"use strict";


(function () {
	
	function _apply () {
		$("html:root > body pre") .dblclick (_selectAll);
		$("html:root > body code") .dblclick (_selectAll);
	}
	
	function _selectAll (_event) {
		var _selection = window.getSelection ();
		_selection.removeAllRanges ();
		var _range = document.createRange ();
		_range.selectNodeContents (this);
		_selection.addRange (_range);
	}
	
	$(window) .on ("load", _apply);
	
}) ();

