+++
title = "hsmusicifier 0.2.0: Add track art and artist info to your Homestuck music collection in any format"
date = "2021-01-23"
category = "announcements"
visible = true
+++
hsmusicifier is a tool to add metadata from [hsmusic.wiki](https://hsmusic.wiki) to a Homestuck music collection. See [the announcement post](https://leo60228.space/hsmusicifier-automatically-add-track-art-to-id3-tags-including-fan-anthologies-and-post-2019-bandcamp/) for more details on the core functionality.

hsmusicifier 0.2.0 adds two major new features.

# Artist info

Ever been annoyed by how the artist of all of your Homestuck tracks just shows up as "Homestuck" despite the enormous multitude of artists? Now, hsmusicifier will add full artist info to all of your tracks, while preserving the Album Artist field used by music players.

![Oops, All Homestuck!](/img/uploads/before.png "Before")

![Full of Artists!](/img/uploads/after.png "After")

# FFmpeg

hsmusicifier now uses the well-known FFmpeg library for reading and writing audio files. This means that it now has support for almost every audio format you could think of! FLAC specifically may be especially useful, due to its combination of losslessness, support for metadata from Bandcamp, and support for track art.

# Download

~~You can download assets and a prebuilt Windows version from [GitHub](https://github.com/leo60228/hsmusicifier/releases/tag/0.2.0).~~

UPDATE (2020-01-24): 0.2.1 has been released, adding a prebuilt Linux version and a few bugfixes. You can (still) get it from [GitHub](https://github.com/leo60228/hsmusicifier/releases/tag/0.2.1).