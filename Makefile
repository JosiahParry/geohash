.PHONY: install bench vendor document

vendor:
	Rscript -e "rextendr::vendor_pkgs()"

document:
	Rscript -e "rextendr::document()"

install:
	Rscript -e "devtools::install()"

bench:
	Rscript bench.R
