# colophon

this iteration is my first attempt at sharing ideas in this manner. i spent a
lot of time thinking about how i could plan for longevity while also keeping
the writing process simple. in the [merveilles](https://merveilles.town)
community, writing your own static site generator is something of a rite of
passage. and while it is fun to some, and can be a fun little challenge, it
represents a deeper idea about building tools that work exactly in the way the
user needs them to work. i considered using a variety of different tools to do
this, but none exactly met my need. in the end, it made more sense to write my
own.

the major requirements for this [[memex]]/wiki were:

- plain text is the primary (on disk format)
- minimal markup outside of markdown
- easily templated
- customizable, going forward, if i want things to change

the plain text is the most important. if later, i want to try to use some other
system, i don't have to import or export any files out of different formats. a
few times i have started working with systems to later find that exporting or
extracting the information i had addedd was considerably more difficult than i
expected. or when it was exported, it wasn't in a very usable format.

the content of this memex itself is in a pretty raw form. you can see the
source code of this memex [on sourcehut](https://git.sr.ht/~metasyn/memex). you
could even use the same tool to write your own if you wanted.

i wrote the first version of the generator in [nim](nimlang.org) but later
rewrote it in rust as a way to learn a little more of the language - just for
fun. along the way, i also added a gemtext generator for those that perfer to
use the [gemini](<https://en.wikipedia.org/wiki/Gemini_(protocol)>) protocol -
see the site at gemini://metasyn.pw using a [gemini client](https://gemini.circumlunar.space/software/).

over time, i also worked on trying to reduce the size of the site and improve
the accessibilty. later i added images, which are by far the biggest impact to
performance. if you have suggestions on accessibility, i would be open to
[[hearing about them|contact]].
