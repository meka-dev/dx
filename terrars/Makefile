all: init proto-gen

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update

	@echo "Copy proto files"
	mkdir -p src/proto
	cp -r ./terrad/proto src/proto/proto
	cp -r ./terrad/third_party/proto src/proto/third_party

proto-gen:
	@echo "Generating Protobuf files"
	cargo run --example proto_gen

.PHONY: all proto-gen init
