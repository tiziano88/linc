package main

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/gorilla/mux"
	p "github.com/tiziano88/linc/server/proto"
)

func main() {
	r := mux.NewRouter()
	r.HandleFunc("/", RootHandler)
	r.HandleFunc("/GetFile", GetFile)
	r.HandleFunc("/UpdateFile", UpdateFile)
	http.ListenAndServe(":8080", r)
}

func RootHandler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello"))
}

func GetFile(w http.ResponseWriter, r *http.Request) {
	rm := p.GetFileRequest{}
	json.NewDecoder(r.Body).Decode(&rm)
	data, err := ioutil.ReadFile(rm.Path)
	if err != nil {
		panic(err)
	}
	rw := p.GetFileResponse{
		JsonContent: string(data),
	}
	json.NewEncoder(w).Encode(rw)
}

func UpdateFile(w http.ResponseWriter, r *http.Request) {
}
