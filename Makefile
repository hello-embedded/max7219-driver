mode=""

WOKWI_ID = ""
example_name = ""
ifeq ($(mode),blinky)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else
	WOKWI_ID = ""
	example_name=example
endif

run:
	export WOKWI_PROJECT_ID=$(WOKWI_ID) ; \
	./scripts/run-wokwi.sh "" $(example_name)
	