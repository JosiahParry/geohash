
<!-- README.md is generated from README.Rmd. Please edit that file -->

# geohash

<!-- badges: start -->
<!-- badges: end -->

`{geohash}` is an R package to provide geohash encoding and decoding. It
is based on the [Rust crate geohash](https://docs.rs/geohash). This
package is actively under development.

You can watch me create this package in my [YouTube
video](https://youtu.be/yaxfqpECIZ0).

## Installation

You can install the development version of geohash like so:

``` r
if (!requireNamespace("remotes")) install.packages("remotes")

remotes::install_github("josiahparry/geohash")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
# geohash doesn't have any exported functions yet
atlanta <- geohash:::encode(-84, 33, 12)
atlanta
#> [1] "dju78x3nfju7"

geohash:::decode(atlanta)
#>     x  y      x_error      y_error
#> 1 -84 33 1.676381e-07 8.381903e-08
```
