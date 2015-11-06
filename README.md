## inquerest



`GET /people?age=lt.13&(student=eq.true|gender=eq.M)&order=age.desc,height.asc&x=1234`

will resolve to 

 Filter:
 `age < 13 AND (student = TRUE OR gender = 'M')`
 
Order:
`age DESC, height ASC`

Query Param:
`x = 1234`


https://github.com/begriffs/postgrest/wiki/Routing