
#' Recipe to geohash encode longitude and latitude
#' 
#' @param recipe recipe
#' @param lon longitude
#' @param lat latitude
#' @param role role
#' @param trained FALSE
#' @param options list(length = 8, parallel = FALSE)
#' @param name "geohash"
#' @param columns NULL
#' @param skip FALSE
#' @param id rand_id("geohash")
#' @export
step_geohash <- function(
  recipe,
  lon = NULL,
  lat = NULL,
  role = NA,
  trained = FALSE,
  options = list(length = 8, parallel = FALSE),
  name = "geohash",
  columns = NULL,
  skip = FALSE,
  id = rand_id("geohash")
) {

   if (!requireNamespace("recipes")) {
       stop("The package `recipes` is required for this functionality")
   }
   
  check_string <- getFromNamespace("check_string", "recipes")

  check_string(name)
  add_step(
    recipe, 
    step_geohash_new(
      lon = enquos(lon),
      lat = enquos(lat),
      role = role,
      trained = trained,      
      options = options,
      name = name,
      columns = columns,
      skip = skip,
      id = id
    )
  )
}

step_geohash_new <- function(
  lon,
  lat,
  role,
  trained,
  options,
  name,
  columns,
  skip,
  id
) {
     if (!requireNamespace("recipes")) {
       stop("The package `recipes` is required for this functionality")
   }
  step(
    subclass = "geohash", 
    lon = lon,
    lat = lat,
    role = role,
    trained = trained,
    options = options,
    name = name,
    columns = columns,
    skip = skip,
    id = id
  )
}

#' @export
prep.step_geohash <- function(
  x,
  training,
  info = NULL,
  ...
) {
  lon_name <- recipes_eval_select(x$lon, training, info)
  lat_name <- recipes_eval_select(x$lat, training, info)
  check_type(training[, c(lon_name, lat_name)], types = c("double", "integer"))
  if (length(lon_name) > 1) {
    cli::cli_abort(c(
      x = "The {.arg lon} selector should select at most a single variable.",
      i = "The following {length(lon_name)} were selected: \\
          {.and {.var {lon_name}}}."
    ))
  }
  check_type(training[, lon_name], types = c("double", "integer"))
  check_type(x$options$length, types = c("double", "integer"))
  check_type(x$options$parallel, types = c("logical"))
  if (length(lat_name) > 1) {
    cli::cli_abort(c(
      x = "The {.arg lat} selector should select at most a single variable.",
      i = "The following {length(lat_name)} were selected: \\
          {.and {.var {lat_name}}}."
    ))
  }
  check_type(training[, lat_name], types = c("double", "integer"))
  step_geohash_new(
    lon = x$lon,
    lat = x$lat,
    role = x$role,
    trained = TRUE,
    options = x$options,
    name = x$name,
    columns = c(lon_name, lat_name),
    skip = x$skip,
    id = x$id
  )
}

#' @export
bake.step_geohash <- function(object, new_data, ...) {
  col_names <- names(object$columns)
  check_new_data(col_names, object, new_data)

  if (length(col_names) == 0) {
    return(new_data)
  }
  length <- object$options$length
  parallel <- object$options$parallel
  if (length(parallel) != 1) {
    parallel = FALSE
  }
  if(parallel[[1]] == FALSE) {
    geohash_vals <-
      encode(
        new_data[[col_names[1]]], # lon
        new_data[[col_names[2]]], # lat
        length
      )
  } else {
    geohash_vals <-
      encode_par(
        new_data[[col_names[1]]], # lon
        new_data[[col_names[2]]], # lat
        length
      )
  }
  new_data[[object$name]] <- geohash_vals
  new_data  
  }



