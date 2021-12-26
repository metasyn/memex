# sfshows

mapping underground music shows in the bay area. see it [here](https://metasyn.pw/shows)

<img src="resources/img/dithered_sfshows.webp" alt="screenshot of metasyn.pw/shows"><img>

## background

[https://metasyn.pw/sfshows](https://metasyn.pw/sfshows) was a project i
started around 2014 to show where various venues are in the san francisco bay
area. the shows are scraped from an amazing resource: [The List](http://www.foopee.com/punk/the-list/).
*the list* has been around for quite some time, and does a great job listing a
lot of shows in a pretty nice, simple, accessible format. like craigslist!

<img src="resources/img/dithered_foopee.webp" alt="screenshot of foopee.com"></img>

however, i found myself forgetting which venues where which, and more
importantly, which ones were on my side of the bay (in san francisco usually).
in order to map these resources, i've manually entered all of the latitude and
longitudes of various venues over the years:

<iframe src="https://git.sr.ht/~metasyn/sfshows/blob/master/src/data/venues.json" width="100%"></iframe>

## parsing

in order to map the resources presented on *the list*, i wrote a small parser
in rust.  previously i had a cronjob that ran this, but finally decided to just
make it a route on its own binary. like other projects, this was my first time
writing any rust code.  the source code can be found
[on sourcehut](https://git.sr.ht/~metasyn/show-scraper-rs).

you can also use the json formatted results of *the list* at
[https://metasyn.pw/show-scraper](https://metasyn.pw/show-scraper).

## plotting

once we have a json list of shows to use - the site uses
[MapLibre](https://maplibre.org/) and react primarily.  originally, the site
was written in vanilla javascript, but i wanted an excuse to learn some react
when i first started encountering it.  i ended up switching the site to use
react just as a learning opportunity, and likely will change it again sometime
when i need to learn some other frontend framework.

you can see the source code of the mapper itself
[on sourcehut](https://git.sr.ht/~metasyn/sfshows).
