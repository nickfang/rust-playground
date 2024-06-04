SRC_DIR := ./
TARGET_DIR := target
RUSTC := rustc

$(TARGET_DIR):
	@echo "Creating target directory..."
	mkdir -p $(TARGET_DIR)

c: $(TARGET_DIR)
	@echo "Compiling $(FILE).rs..."
	$(RUSTC) $(SRC_DIR)/$(FILE).rs -o $(TARGET_DIR)/$(FILE)
	@echo "Compilation finished. The output file is $(TARGET_DIR)/$(FILE)."

run: c
	@echo "Running the compiled program..."
	./$(TARGET_DIR)/$(FILE)

clean:
	@echo "Cleaning up..."
	rm -rf $(TARGET_DIR)
	@echo "Cleanup finished."

.PHONY: c run clean