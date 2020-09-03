# path macros
BIN_PATH := bin
BUILD_PATH := _build
SRC_PATH := src

# compile macros
TARGET_NAME := spawn-chain
TARGET := $(BIN_PATH)/$(TARGET_NAME)

SRC := $(foreach x, $(SRC_PATH), $(wildcard $(addprefix $(x)/*,.erl*)))

# default rule
default: all

# non-phony targets
$(TARGET): $(SRC) $(BIN_PATH) $(BUILD_PATH)
	lumen compile --output $@ --output-dir $(BUILD_PATH) $(SRC_PATH)

$(BIN_PATH):
	mkdir  $@

$(BUILD_PATH):
	mkdir $@

# phony targets
.PHONY: all
all: $(TARGET)

.PHONY: clean
clean:
	rm -f $(wildcard _build/*)
	rm -f $(TARGET)

test: $(TARGET)
	$(TARGET) 200
