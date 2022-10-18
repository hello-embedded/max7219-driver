mode=""

WOKWI_ID = ""
example_name = ""
ifeq ($(mode),max7219)
	WOKWI_ID = "345754769993761364"
	example_name=$(mode)
else ifeq ($(mode),devices)
	WOKWI_ID = "345754769993761364"
	example_name=$(mode)
else
	WOKWI_ID = ""
	example_name=example
endif

run:
	export WOKWI_PROJECT_ID=$(WOKWI_ID) ; \
	./scripts/run-wokwi.sh "" $(example_name)
	