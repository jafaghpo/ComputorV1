NAME = computor

all: build_release $(NAME)

build_release:
	@cargo build --release

$(NAME):
	@ln -s target/release/$(NAME)

build_debug:
	@cargo build

link_debug:
	@rm $(NAME)
	@ln -s target/debug/$(NAME)

debug: build_debug link_debug

test:
	@cargo test

clean: 
	@rm -rf $(NAME)
	@rm -rf target

re: clean all
.PHONY: all clean re
