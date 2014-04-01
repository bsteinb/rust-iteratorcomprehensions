RUSTCRATES = iteratorcomprehensions examples

iteratorcomprehensions_RUSTCFLAGS += --allow unused_variable

examples_CRATE_DEPS += iteratorcomprehensions
examples_RUSTCFLAGS += --allow unused_variable

include rust-mk/rust.mk
