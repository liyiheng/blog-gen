package main

import (
	"log"
	"net/http"
	"os"
	"html/template"
	//for extracting service credentials from VCAP_SERVICES
	//"github.com/cloudfoundry-community/go-cfenv"
	"io/ioutil"
	"strings"
)

const (
	DEFAULT_PORT = "8080"
)

var index = template.Must(template.ParseFiles(
	"templates/_base.html",
	"templates/index.html",
))

const URL string = "https://www.google.com.hk"

func helloworld(w http.ResponseWriter, req *http.Request) {
	scheme := "http://"
	if req.TLS != nil {
		scheme = "https://"
	}
	uurl := strings.Join([]string{scheme, "www.google.com", req.RequestURI}, "")
	resp, err := http.Get(uurl)
	if err != nil {
		index.Execute(w, nil)
		return
	}
	data, err := ioutil.ReadAll(resp.Body)
	defer resp.Body.Close()
	w.Write(data)
}

func main() {
	var port string
	if port = os.Getenv("PORT"); len(port) == 0 {
		port = DEFAULT_PORT
	}

	http.HandleFunc("/", helloworld)
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))

	log.Printf("Starting app on port %+v\n", port)
	http.ListenAndServe(":" + port, nil)
}
