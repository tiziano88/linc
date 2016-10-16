package main

import (
	"io/ioutil"
	"net/http"
	"os"

	"github.com/golang/protobuf/jsonpb"
	"github.com/gorilla/mux"
	p "github.com/tiziano88/linc/server/proto"
)

var (
	m = &jsonpb.Marshaler{}
)

func main() {
	r := mux.NewRouter()
	r.HandleFunc("/", RootHandler)
	r.HandleFunc("/LoadFile", GetFile)
	r.HandleFunc("/SaveFile", UpdateFile)
	http.ListenAndServe(":8080", r)
}

func RootHandler(w http.ResponseWriter, r *http.Request) {
	http.ServeFile(w, r, "./client/out/index.html")
}

func GetFile(w http.ResponseWriter, r *http.Request) {
	rm := &p.GetFileRequest{}
	jsonpb.Unmarshal(r.Body, rm)
	rm.Path = "/tmp/src.json"
	data, err := ioutil.ReadFile(rm.Path)
	if err != nil {
		panic(err)
	}
	rw := &p.GetFileResponse{
		JsonContent: string(data),
	}
	err = m.Marshal(w, rw)
	if err != nil {
		panic(err)
	}
}

func UpdateFile(w http.ResponseWriter, r *http.Request) {
	rm := &p.UpdateFileRequest{}
	jsonpb.Unmarshal(r.Body, rm)
	rm.Path = "/tmp/src.json"
	err := ioutil.WriteFile(rm.Path, []byte(rm.JsonContent), os.ModePerm)
	if err != nil {
		panic(err)
	}
}
