

(list
	
	(and)
	(and #t)
	(and #f)
	(and 1 2)
	
	(and #t #f)
	(and #f #t)
	
	(and 1 2 #f)
	(and #f 1 2)
	
	(and #t (list))
	(and (list) #t)
	
	(and #f (list))
	(and (list) #f)
	
	(and 1 2 (list))
	(and (list) 1 2)
)




(list
	
	(or)
	(or #t)
	(or #f)
	(or 1 2)
	
	(or #t #f)
	(or #f #t)
	
	(or 1 2 #f)
	(or #f 1 2)
	
	(or #t (list))
	(or (list) #t)
	
	(or #f (list))
	(or (list) #f)
	
	(or 1 2 (list))
	(or (list) 1 2)
)

