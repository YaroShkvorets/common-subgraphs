.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run graph_out -e eth.substreams.pinax.network:9000 -s -1000 -o jsonl

.PHONY: gui
gui:
	substreams gui graph_out -e eth.substreams.pinax.network:9000 -s -1000


.PHONY: graph-build
graph-build:
	graph build

.PHONY: graph-create
graph-create:
	graph create --node http://localhost:8020/ substreams-entities

.PHONY: graph-deploy
graph-deploy:
	graph deploy --node http://localhost:8020/ --ipfs http://localhost:5001 substreams-entities

