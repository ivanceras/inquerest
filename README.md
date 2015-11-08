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

##Similar projects

* [rustless/queryst](https://github.com/rustless/queryst)



##TODO
select=id,name,age,address,max(sum(grade))
&from=student,person
&left_join=school_student
&on=school_student.id,student.id
&on=school.id=school_student.id


select=*
&inner_join=school,student

&using=school.id
