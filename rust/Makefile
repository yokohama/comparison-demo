PROFILE = yokohama

build:
	cargo lambda build --release

fn-deploy:
	cargo lambda deploy rust --profile $(PROFILE)

fn-invoke:
	cargo lambda invoke --remote --data-ascii '{"command": "Hello world"}' rust --profile $(PROFILE)

sam:
	sam local invoke RustFunction --event event.json --profile yokohama
