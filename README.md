# DotMatrix

a very rough project where i learned to drive four LED dot-matrices through GPIO on the raspberry pi pico, using [embassy](https://embassy.dev) and embedded rust.

for now this project is a somewhat disorganized sandbox to play with.

one day, i would like to revisit the individual drivers for each electronic component and make them more modular, so i could reuse them in future projects. 

![photo](https://raw.githubusercontent.com/d3npa/dotmatrix/d0c4678f8c181001f44343a2eed1ee27ce014fad/photo.jpg)

### before compiling

create a `credentials.rs` file in the project root containing the following 
two lines:

```rs
const WIFI_NETWORK: &str = "";
const WIFI_PASSWORD: &str = "";
```

it will be including during the build process.

