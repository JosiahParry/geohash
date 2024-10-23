test_that("step_geohash works", {
  testthat::skip_if_not_installed("recipes")
  library(recipes)
  n <- 10
  df <- data.frame(
    y = rnorm(n, 0, 1),
    longitude  = runif(n, -180, 180),
    latitude = runif(n, -90, 90)
  )
  rec <- df |> 
    recipe(y ~ .) |> 
    step_geohash(
      lon = longitude,
      lat = latitude,
      name = "encoded",
      options = list(length = 5)
  )
  juiced <- rec |> prep() |> juice()

  expect_s3_class(juiced, "data.frame")  
  expect_named(juiced, c("longitude", "latitude", "y", "encoded"))
  expect_true(nrow(juiced) == n)
  expect_type(juiced[["encoded"]], "character")
  expect_true(nchar(juiced[["encoded"]][[1]]) == 5)
})