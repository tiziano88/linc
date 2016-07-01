CLIENT_DIR=client
SERVER_DIR=server
PROTO_DIR=proto
OUT_DIR=out

SERVER_PROTO=$(SERVER)/$(PROTO_DIR)/server.pb.go

INDEX=$(CLIENT_DIR)/$(OUT_DIR)/index.html
SERVER=$(SERVER_DIR)/$(OUT_DIR)/linc

all: $(INDEX) $(SERVER)

$(INDEX): $(CLIENT_DIR)/*
	elm make $(CLIENT_DIR)/main.elm --output $(INDEX)

$(SERVER): $(SERVER_DIR)/* $(SERVER_PROTO)
	go build -o $(SERVER) $(SERVER_DIR)/main.go

$(SERVER_PROTO): $(PROTO_DIR)/*.proto
	protoc --go_out=$(SERVER_DIR) --elm_out=$(CLIENT_DIR) $(PROTO_DIR)/*.proto

run: $(INDEX) $(SERVER)
	$(SERVER)
