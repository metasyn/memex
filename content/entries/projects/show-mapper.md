# show-mapper

<!--epistemic=dendroid-->

mapping underground music shows in the bay area. see it [here](https://shows.metasyn.pw)

<img src="resources/img/dithered_show-mapper.png" alt="screenshot of shows.metasyn.pw"><img>

## background

[https://shows.metasyn.pw](https://shows.metasyn.pw) was a project i
started around 2014 to show where various venues are in the san francisco bay
area. the shows are scraped from an amazing resource: [The List](http://www.foopee.com/punk/the-list/).
_the list_ has been around for quite some time, and does a great job listing a
lot of shows in a pretty nice, simple, accessible format. like craigslist!

<img src="resources/img/dithered_foopee.png" alt="screenshot of foopee.com"></img>

however, i found myself forgetting which venues where which, and more
importantly, which ones were on my side of the bay (in san francisco usually).
in order to map these resources, i've manually entered all of the latitude and
longitudes of various venues over the years. eventually this became unsustainable, and i figured out a way to semi automate looking them up with relatively reasonable accuracy.

## parsing

in order to map the resources presented on _the list_, i wrote a small parser
in rust. previously i had a cronjob that ran this, but finally decided to just
make the parsing dynamic when accesing the map. this was actually my first time writing rust code, and turned me on to using the language. pretty fun.

years later (2023), i added parsing for [19hz.info](https://19hz.info), another lightweight site for finding local shows and music. they have a bay area page, so i parse that information as well and also try to plot it on the same map.

## plotting

once we have a json list of shows to use - the site uses
[MapLibre](https://maplibre.org/). the first version was vanilla javascript, but poorly organized. sometime around 2017 i rewrote the whole thing in react, mostly just to learn react at the time, which i was sometimes needing to use at work. in 2023, i rewrote the whole thing to use vanilla typescript, which allowed me to vastly simplify the entire codebase and also get some type safety. its amazing how helpful simple type checking can be.
