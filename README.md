# stuart-math

A simple [Stuart](https://github.com/w-henderson/Stuart) plugin for LaTeX math rendering.

The plugin currently uses [mathjax_svg](https://github.com/gw31415/mathjax_svg), which in turn packages the entirety of V8 just to render math. This is obviously not ideal - I think at some point I'll add an optional Stuart feature to put V8 in Stuart and allow plugins directly written in JavaScript - but for the time being this will do. It's not that slow, it just makes the plugin a bit large.