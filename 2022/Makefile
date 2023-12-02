.PHONY: is-number
is-number:
	@case ${NUM} in ''|*[!0-9]*) exit 1;; *);; esac

.PHONY: run-%
run-%:
	@$(MAKE) is-number NUM=$(*)
	(cargo run --bin day$(*) > tmp/day$(*)/debug.log); bat tmp/day$(*)/debug.log

.PHONY: test-%
test-%:
	@$(MAKE) is-number NUM=$(*)
	cargo test --bin day$(*) -- --nocapture

.PHONY: init-%
init-%:
	@$(MAKE) is-number NUM=$(*)
	@[ ! -z "${SRC}" ] || (echo "Please specify SRC." && exit 1)
	@([ -d src/day$(*) ] && echo "src/day$(*) already exists") || cp -r src/day${SRC} src/day$(*)
	@([ -d tmp/day$(*) ] && echo "tmp/day$(*) already exists") || cp -r tmp/day${SRC} tmp/day$(*)

.PHONY: input-%
input-%:
	@$(MAKE) is-number NUM=$(*)
	@bat tmp/day$(*)/input.txt
