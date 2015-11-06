## Inquerest

[![Build Status](https://travis-ci.org/ivanceras/inquerest.svg?branch=master)](https://travis-ci.org/ivanceras/inquerest)

`GET /people?age=lt.13&(student=eq.true|gender=eq.M)&order=age.desc,height.asc&x=1234`

will resolve to 

* Filters:

    `age < 13 AND (student = TRUE OR gender = 'M')`
 
* Order by:

    `age DESC, height ASC`

* other params:

    `x = 1234`


Inspired by [postgrest](https://github.com/begriffs/postgrest) [filter expressions](https://github.com/begriffs/postgrest/wiki/Routing)
