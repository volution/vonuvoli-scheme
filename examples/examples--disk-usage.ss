
(define size
	(fs-directory-fold-recursive
		"/usr/share/doc"
		(lambda (accumulator entry metadata)
			(if (fs-metadata-file? metadata)
				(begin
					; (write-line (path->string entry))
					(+ accumulator (fs-metadata-file-size metadata)))
				accumulator))
		#f ; recurse always
		0 ; accumulator
		#t ; join parent and entry
		#f ; do not return entry kind
		#t ; return entry metadata
		#t ; follow symlinks
	))

(write-line size)

