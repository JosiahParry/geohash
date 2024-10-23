n <- 1e6
x <- runif(n, -180, 180)
y <- runif(n, -90, 90)

bench::mark(
  geohash = geohash::encode(x, y, 8L),
  geohash_par = geohash::encode_par(x, y, 8L),
  geohashTools = geohashTools::gh_encode(y, x, 8L),
  iterations = 25L
)
#> # A tibble: 3 × 13
#>   expression     min  median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
#>   <bch:expr> <bch:t> <bch:t>     <dbl> <bch:byt>    <dbl> <int> <dbl>   <bch:tm>
#> 1 geohash    12.53ms 12.89ms      74.2    1.54MB     0       25     0      337ms
#> 2 geohash_p…  5.41ms  5.61ms     162.   783.72KB     0       25     0      155ms
#> 3 geohashTo…  8.79ms  9.08ms     108.   816.73KB     4.49    24     1      223ms

library(geohash)
library(recipes)
n <- 1e6
x <- runif(n, -180, 180)
y <- runif(n, -90, 90)
df <- data.frame(
  y = rnorm(n, 0, 1),
  longitude  = x,
  latitude = y
)

run_step_geohash  <- \(parallel) {
  rec <- df |> 
    recipe(y ~ .) |> 
    geohash::step_geohash(
      lon = longitude,
      lat = latitude,
      name = "geohash",
      options = list(length = 8L, parallel = parallel)
  )
  juiced <- rec |> prep() |> juice()
  juiced
}

step_mutate_encode_par  <- \() {
  rec <- df |> 
    recipe(y ~ .) |> 
    step_mutate(
      geohash = geohash::encode_par(longitude, latitude, length = 8L)
    )
  juiced <- rec |> prep() |> juice()
  juiced
}

bench::mark(
  step_geohash = run_step_geohash(parallel = FALSE),
  step_geohash_par = run_step_geohash(parallel = TRUE),
  step_mutate_encode_par = step_mutate_encode_par(),
  min_time = Inf,
  iterations = 15L,
  memory = FALSE,
  time_unit = "s"
)[,c("expression", "median", "n_itr" , "total_time")]
# # A tibble: 3 × 4
# expression             median n_itr total_time
# <bch:expr>              <dbl> <int>      <dbl>
# 1 step_geohash             3.88    12       46.6
# 2 step_geohash_par         3.88    11       42.7
# 3 step_mutate_encode_par   3.88    11       42.9