remove-target:
	@echo "remove target..."
	@find . -type d -name "target" -print0 | xargs -0 rm -rf

find-target:
	@echo "find target..."
	find . -name "target" -print -type f