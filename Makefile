.PHONY: is-number run test init input

is-number:
	@case ${NUM} in ''|*[!0-9]*) exit 1;; *);; esac

run: guard-YEAR guard-DAY
	@$(MAKE) is-number NUM=${DAY}
	@$(MAKE) is-number NUM=${YEAR}
	@(cargo run --bin "${YEAR}day${DAY}" -- --input=src/${YEAR}/tmp/day${DAY}/input.txt > src/${YEAR}/tmp/day${DAY}/debug.log); bat src/${YEAR}/tmp/day${DAY}/debug.log

test: guard-YEAR guard-DAY
	@$(MAKE) is-number NUM=${DAY}
	@$(MAKE) is-number NUM=${YEAR}
	cargo test --bin "${YEAR}day${DAY}" -- --nocapture

input: guard-YEAR guard-DAY
	@$(MAKE) is-number NUM=${DAY}
	@$(MAKE) is-number NUM=${YEAR}
	@bat tmp/day$(*)/input.txt

guard-%:
	@[ -n $(*) ] || exit 1
