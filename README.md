
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

Encode x and y coordinates as a geoash using `encode()`

``` r
library(geohash)

# create random x & y points
n <- 5
x <- runif(n, -180, 180)
y <- runif(n, -90, 90)


gh <- encode(x, y, 8)
gh
#> [1] "pnmw98xy" "fhezzn3t" "fypr27n5" "st4ssv40" "xk0s0nc1"
```

The geohashes can be decoded using `decode()` which provides you with
their center, and the error in both the x and y directions.

``` r
decode(gh)
#> # A data frame: 5 × 4
#>       x     y  x_error   y_error
#>   <dbl> <dbl>    <dbl>     <dbl>
#> 1 143.  -53.7 0.000172 0.0000858
#> 2 -84.4  71.7 0.000172 0.0000858
#> 3 -46.0  80.0 0.000172 0.0000858
#> 4  26.2  28.9 0.000172 0.0000858
#> 5 147.   23.2 0.000172 0.0000858
```

Additionally, you can extract the bounding box of the geohashes using
`decode_bbox()`. This returns a `bbox` object from the `sf` package.

``` r
bboxes <- decode_bbox(gh) 
bboxes
#> [[1]]
#>      xmin      ymin      xmax      ymax 
#> 142.81094 -53.69740 142.81128 -53.69722 
#> 
#> [[2]]
#>      xmin      ymin      xmax      ymax 
#> -84.41689  71.71000 -84.41654  71.71017 
#> 
#> [[3]]
#>      xmin      ymin      xmax      ymax 
#> -46.03546  80.04141 -46.03512  80.04158 
#> 
#> [[4]]
#>     xmin     ymin     xmax     ymax 
#> 26.22711 28.94348 26.22746 28.94365 
#> 
#> [[5]]
#>      xmin      ymin      xmax      ymax 
#> 146.95450  23.24038 146.95484  23.24055
```

You can use these bounding boxes to create an `sfc` of polygons.

``` r
do.call(c, lapply(bboxes, sf::st_as_sfc))
#> Geometry set for 5 features 
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -84.41689 ymin: -53.6974 xmax: 146.9548 ymax: 80.04158
#> Geodetic CRS:  WGS 84
#> POLYGON ((142.8109 -53.6974, 142.8113 -53.6974,...
#> POLYGON ((-84.41689 71.71, -84.41654 71.71, -84...
#> POLYGON ((-46.03546 80.04141, -46.03512 80.0414...
#> POLYGON ((26.22711 28.94348, 26.22746 28.94348,...
#> POLYGON ((146.9545 23.24038, 146.9548 23.24038,...
```

Alternatively, you can identify the neighboring geohash cells using
`neighbor()` and `neighbors()` function. The `neighbor()` function
identifies a single neighbor based on a direction whereas `neighbors()`
provides all adjacent geohashes.

Here we can find the neighbors to the north and south-east.

``` r
neighbor(gh, "n")
#> [1] "pnmw98xz" "fhezzn3w" "fypr27nh" "st4ssv41" "xk0s0nc4"
```

``` r
neighbor(gh, "se")
#> [1] "pnmw9b8j" "fhezzn3u" "fypr27n6" "st4ssufr" "xk0s0nc2"
```

We can also find *all* adjacent neighbors.

``` r
neighbors(gh)
#> # A data frame: 5 × 8
#>   n        ne       e        se       s        sw       w     nw   
#>   <chr>    <chr>    <chr>    <chr>    <chr>    <chr>    <chr> <chr>
#> 1 pnmw98xz pnmw9b8p pnmw9b8n pnmw9b8j pnmw98xv pnmw98xt pnmw… pnmw…
#> 2 fhezzn3w fhezzn3y fhezzn3v fhezzn3u fhezzn3s fhezzn3k fhez… fhez…
#> 3 fypr27nh fypr27nk fypr27n7 fypr27n6 fypr27n4 fypr27jf fypr… fypr…
#> 4 st4ssv41 st4ssv43 st4ssv42 st4ssufr st4ssufp st4ssucz st4s… st4s…
#> 5 xk0s0nc4 xk0s0nc6 xk0s0nc3 xk0s0nc2 xk0s0nc0 xk0s0nbb xk0s… xk0s…
```
