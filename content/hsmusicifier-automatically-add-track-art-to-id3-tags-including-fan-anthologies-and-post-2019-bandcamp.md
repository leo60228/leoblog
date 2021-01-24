+++
title = "hsmusicifier: Automatically add track art to ID3 tags (including fan anthologies and post-2019 Bandcamp)"
date = "2021-01-21"
category = "announcements"
visible = true
+++
(NOTE: this post was originally on [Reddit](https://old.reddit.com/r/homestuck/comments/l2ekhj/hsmusicifier_automatically_add_track_art_to_id3/) before this blog was created)

hsmusicifier is a tool to add track art from the wonderful [hsmusic.wiki](https://hsmusic.wiki) to ID3 tags. ID3 is a way to add metadata to songs generally used (by both Bandcamp and the vast majority of music players) for mp3 files. This doesn't sound that helpful at first, but it doesn't *just* work for albums that originally had track art. [hsmusic.wiki](https://hsmusic.wiki) also contains multiple fan anthologies of track art and has the SoundCloud exclusive track art for the Friendsim OST!

Also, if you've recently purchased the Homestuck albums, you won't have track art (well, except for coloUrs and mayhem: Universe A). hsmusicifier can help you too! All albums on the current Bandcamp are supported by hsmusicifier. For albums that never had track art of any kind, hsmusicifier will still add the art for each individual album instead of the global art used for the entire collection.

In theory, hsmusicifier will work for all albums with track art on [hsmusic.wiki](https://hsmusic.wiki), along with the 2019 collection albums. However, I haven't been able to test every one of them. These are the ones I've tested:

* Alterniabound (with Alternia)
* Beyond Canon
* coloUrs and mayhem: Universe A & B
* HIVESWAP Act 1 OST (with THE GRUBBLES)
* Hiveswap Friendsim
* Homestuck Vol. 1-4 (with Midnight Crew: Drawing Dead)
* Homestuck Vol. 5-6 (with The Felt)
* Homestuck Vol. 7-8 (with Cherubim)
* Homestuck Vol. 9-10 (with \[S] Collide. and Act 7)
* Symphony Impossible to Play (with Medium)

In addition, while these albums were always released with their original track art (whether existing or not), they're much less useful, but I've still confirmed that the tool is compatible:

* HIVESWAP Act 2 Original Soundtrack
* Diverging Delicacies
* Land of Fans and Music 2
* Land of Fans and Music 3
* Land of Fans and Music 4
* Weird Puzzle Tunes

I suppose this could be useful for the LOFAM albums if your download somehow got corrupted.

Anyway, you can download the tool on [GitHub](https://github.com/leo60228/hsmusicifier/releases/latest). The release includes the required data files as well as a Windows version. I've tested Linux built from source (that's how I did development, in fact). macOS should work too, but I haven't tested it.