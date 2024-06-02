SRC_DIR := ./
TARGET_DIR := target
RUSTC := rustc

$(TARGET_DIR):
	mkdir -p $(TARGET_DIR)

c: $(TARGET_DIR)
	$(RUSTC) $(SRC_DIR)/$(FILE).rs -o $(TARGET_DIR)/$(FILE)

clean:
	rm