.onLoad <- function(libname, pkgname) {  
  if(requireNamespace("recipes")) {
    prep.step_geohash <<- prep_step_geohash_impl
    bake.step_geohash <<- bake_step_geohash_impl
  }
}