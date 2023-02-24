# show-mapper

<!--epistemic=dendroid-->

mapping underground music shows in the bay area since 2014. see it [here](https://shows.metasyn.pw).

<img src="resources/img/dithered_show-mapper.png" alt="screenshot of shows.metasyn.pw"><img>

## background

[https://shows.metasyn.pw](https://shows.metasyn.pw) was a project i started
around 2014 to show where various venues are in the san francisco bay area. i
had just moved to the city and remembered this cool site my cousin had showed
me she called [the list](http://www.foopee.com/punk/the-list/) (maintained by
graham spencer). _the list_ has been around for quite some time, and does a
great job listing a lot of shows in a pretty nice, simple, accessible format.
like craigslist! interestingly, the list is actually just an html version of <a
href="mailto:skoepke@calweb.com"> Steve Koepke's</a> excellent aggregation,
which you can subscribe to [here](https://stevelist.com/). there are a few
other versions out there, like jon luini's version
[here](http://jon.luini.com/thelist/date.html), or the mobile friendly version
[here](https://www.riotlist.com/sf/). if you're interested, you can read more about
[the history of the list](https://bayareapunk.com/blog/what-is-the-list.html).
despite the multiplicity, none of them put the shows on a map. usually when i'm
looking up shows, i know two things: when i'll be free to go to one, and how
far i'm interested in traveling. the map is useful for me since i often want to
know about shows in a particular area in particular, especially when its more
of an impromptu decision to go to one.

<img src="resources/img/dithered_foopee.png" alt="screenshot of foopee.com"></img>

however, i found myself forgetting which venues where which, and more
importantly, which ones were on my side of the bay (in san francisco usually).
in order to map these resources, i've manually entered all of the latitude and
longitudes of various venues over the years. eventually this became
unsustainable, and i figured out a way to semi automate looking them up with
relatively reasonable accuracy.

## parsing

in order to map the resources presented on _the list_, i wrote a small parser
in rust. previously i had a cronjob that ran this, but finally decided to just
make the parsing dynamic when accessing the map. this was actually my first time
writing rust code, and turned me on to using the language. pretty fun.

years later (2023), i added parsing for [19hz.info](https://19hz.info), another
lightweight site for finding local shows and music. they have a bay area page,
so i parse that information as well and also try to plot it on the same map.

## plotting

once we have a json list of shows to use - the site uses
[MapLibre](https://maplibre.org/). the first version was vanilla javascript,
but poorly organized. sometime around 2017 i rewrote the whole thing in react,
mostly just to learn react at the time, which i was sometimes needing to use at
work. in 2023, i rewrote the whole thing to use vanilla typescript, which
allowed me to vastly simplify the entire codebase and also get some type
safety. its amazing how helpful simple type checking can be. i think the whole thing
loads a lot faster now.
