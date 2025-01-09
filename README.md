This is my fork of [rust-mpd](https://github.com/kstep/rust-mpd) which
I use in my MPD client, [inori](https://github.com/eshrh/inori).

The original program is licensed under either Apache v2 or MIT. I have
chosen the MIT license, and you can find the original
license/copyright notice [here](./LICENSE-MIT), included with the
software.

Features I have implemented are not particularly idiomatic or well
written, but they get ~~the~~ *my* job done. New additions are:
- `fn list_group_2(&mut self, terms: (String, String)) ->
  Result<Vec<(String, String)>>` which calls the "list" command for
  two terms with the group keyword, as specified in the
  [protocol](https://mpd.readthedocs.io/en/latest/protocol.html#the-music-database)
- `fn list_groups(&mut self, terms: Vec<&str>) ->
  Result<Vec<Vec<String>>>` which also calls the "list" command for an
  arbitrary number of terms. The output nested vectors contains the
  grouping structure. So, for example, a call like "title group album"
  will return
  ```
  [["album1"], ["album1", "title1"], ["album2"], ["album2 title2"]]
  ```
  and so on. This is primarily useful to obtain
  one entry for each object in the library; I use it in inori for the
  global search feature.
- `Client<StreamTypes>` type which supports both `TcpStream` and
  `UnixStream` connections. This type has implementations
  `connect_tcp`, `connect_socket`, `connect` (which tries both), and
  also `default` which implements the default fallbacks specified [by
  the mpd doc](https://mpd.readthedocs.io/en/latest/client.html#connecting-to-mpd)

With that said, my bar for quality is honestly quite low, I'd rather
save anybody else the trouble of having to make their own fork to
add a simple convenience feature. Feel free to send me any patches
and I'll almost certainly accept them quickly as long as they don't
break anything!

It is not possible to implement these features that I need without a
fork because the necessary Client api is not public. Besides, it is
not right for an mpd library to implement usecase specific commands
that are not defined in the spec.

If you'd like to use this fork as a drop-in replacement, you can do
that the same way I do it in inori:
```
[dependencies.mpd]
package = "inori-mpd"
version = "0.1.0"
```
