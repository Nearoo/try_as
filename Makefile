.PHONY: docs

docs:
	cargo doc
	rm -rf docs/*
	mkdir -p docs
	cp -r target/doc/* docs/.
	echo "<meta http-equiv=\"refresh\" content=\"0; url=try_as\">" > docs/index.html