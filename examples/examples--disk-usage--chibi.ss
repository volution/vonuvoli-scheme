
(import (scheme base))
(import (scheme write))
(import (chibi filesystem))


(define size
	(directory-fold-tree
		"/usr/share/doc"
		#f #f
		(lambda (path accumulator)
			(define metadata (file-status path))
			(if (and metadata (file-regular? metadata))
				(begin
					; (write path) (newline)
					(+ accumulator (file-size metadata)))
				accumulator))
		0 ; accumulator
	))

(write size)
(newline)

