
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

What is special about using `README.Rmd` instead of just `README.md`?
You can include R chunks like so:

``` r
summary(cars)
#>      speed           dist       
#>  Min.   : 4.0   Min.   :  2.00  
#>  1st Qu.:12.0   1st Qu.: 26.00  
#>  Median :15.0   Median : 36.00  
#>  Mean   :15.4   Mean   : 42.98  
#>  3rd Qu.:19.0   3rd Qu.: 56.00  
#>  Max.   :25.0   Max.   :120.00
```

You’ll still need to render `README.Rmd` regularly, to keep `README.md`
up-to-date. `devtools::build_readme()` is handy for this.

You can also embed plots, for example:

<img src="man/figures/README-pressure-1.png" style="width:100.0%" />

In that case, don’t forget to commit and push the resulting figure
files, so they display on GitHub and CRAN.
