# OVRT-SYS &emsp; [![Build Status]][actions] [![Doc]][docurl]

[build status]: https://img.shields.io/github/workflow/status/swfsql/ovrt-sys/Rust/master
[actions]: https://github.com/swfsql/ovrt-sys/actions?query=branch%3Amaster
[doc]: https://img.shields.io/badge/-doc-brightgreen
[docurl]: https://swfsql.github.io/ovrt-sys/doc/ovrt_sys/index.html

Bindings for ovr-toolkit custom apps js api.

Based on [wiki/CustomApps](http://wiki.ovrtoolkit.co.uk/index.php?title=CustomApps&oldid=390)
and on [Zetaphor/ovrt-helper](https://github.com/Zetaphor/ovrt-helper/blob/524815728c19d431431f2f7a9a6e3ca7a6a19691/ovrt-helper.js).

## On/Offline Testing

TODO

## Online Testing (Outdated)

To test the latest working master branch, paste into your `C:\Program Files (x86)\Steam\steamapps\common\OVR Toolkit\customAppDebug.txt`:

```
https://swfsql.github.io/ovrt-sys/
```

And then re/start ovrt.

You may also open [that link](https://swfsql.github.io/ovrt-sys/) in your browser, but it will fail when trying to call a function that would have been defined by ovrt itself.

## Offline Build Testing (Outdated)

You'll need a [rust nightly toolchain](https://www.rust-lang.org/tools/install), [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html) (no need to install anything related to npm) and a file server such as [http](https://github.com/thecoshman/http#installation).

```bash
git clone https://github.com/swfsql/ovrt-sys.git
cd ovrt-sys
wasm-pack build --target web
http
```

And for testing it, point your `C:\Program Files (x86)\Steam\steamapps\common\OVR Toolkit\customAppDebug.txt` to:

```
http://localhost:8000/
```

And then re/start ovrt.

You may also open [that link](http://localhost:8000/) in your browser, but it will fail when trying to call a function that would have been defined by ovrt itself.

## TODO Application Building

TODO

- TODO (to build your own application, take a look at example blablabla..)
- TODO (for druid usage, apply the "druid" feature..)
- TODO (for iced usage, apply the "iced" feature..)
- TODO (for yew usage, apply the "yew" feature..)
- TODO etc..

## Useful Links

- [druid/examples/hello_web](https://github.com/linebender/druid/tree/master/druid/examples/hello_web)
- [wasm-pack book](https://rustwasm.github.io/docs/wasm-pack/)
- [wasm-bindgen book](https://rustwasm.github.io/docs/wasm-bindgen/)
