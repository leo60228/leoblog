+++
title = "Automatic syndication and cross-platform interactions with blogs"
date = "2021-02-05"
category = "notes"
visible = true

[extra]
mastodon = "https://60228.dev/@leo/105681362437128155"
+++
I've set up the following functionality:

* <https://leo60228.space> and <https://leo60228.tumblr.com> will notify linked sites (outgoing Webmentions)
* Posts from <https://leo60228.space> will automatically be posted on Mastodon (syndication)
* External interactions with posts on <https://leo60228.space> and <https://leo60228.tumblr.com> will be displayed on the posts (incoming Webmentions, Webmention bridging)
  * Links to posts on Twitter
  * Replies to automatic cross-posts on Mastodon
  * Links from Webmention-compatible sites

## Outgoing Webmentions for leo60228.space

There's quite a few ways to go about implementing support for outgoing Webmentions. My method replies on external services, but required very few changes to my site to set up.

First, I had to implement support for microformats2 on my site. This was easy enough to do: I just had to add some classes to elements that described different (meta)data about my site and posts, as well as add a `<link rel="author">` to my homepage on posts. This required some minor restructuring of my markup.

Next, I had to actually send out Webmentions. With [webmention.app](https://webmention.app), this was very easy. All I had to do was sign up for an account and add a webhook whenever my site was deployed that included a link to a pre-existing Atom feed. [This is documented pretty well.](https://webmention.app/docs#how-to-integrate-with-netlify)

## Syndication and Webmention bridging

Syndication was made very easy by [Bridgy](https://www.brid.gy). With Bridgy, all I had to do was sign in Mastodon, and then I can automatically syndicate a post from my blog to Mastodon just by including a Webmention to a special link.

Webmention bridging was just as easy. All I had to do was sign in with the services that I'd like to integrate with. There are a few caveats here, however, which are listed in the Bridgy documentation. The most important one is that due to Twitter API limitations some mentions may be lost.

## Incoming Webmentions for leo60228.space

There are also quite a few approaches here, however I went with the popular [Webmention.io](https://webmention.io) which provides a hosted service and API for receiving Webmentions (the IndieWeb people seem to be very good at naming things). Webmention.io's support for displaying interactions on sites is limited to a simple counter, however I found [webmention.js](https://github.com/PlaidWeb/webmention.js) which provides a simple display of Webmentions.

## Webmentions for leo60228.tumblr.com

Bridgy makes *outgoing* Webmentions simple enough, just by signing in with Tumblr on the Bridgy site. Incoming Webmentions are also possible, but a bit trickier. Webmentions *would* map to comments, but Tumblr only has reblogs, which function a bit differently. Bridgy handles this by using Disqus, external commenting software that has bidirectional integration with Tumblr.

This works well enough, but adding an entire comments feature seems a bit heavy to me. It'd be interesting if there was a way to *just* display Webmentions within a Tumblr theme. I'm not sure of the feasibility or difficulty of something like this, though.

<a href="https://brid.gy/publish/mastodon"></a>
