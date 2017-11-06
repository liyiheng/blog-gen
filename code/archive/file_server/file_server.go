//usr/bin/env go run "$0" "$@"; exit "$?"
package main

import (
	"fmt"
	"net"
	"net/http"
	"os"
	"os/signal"
	"syscall"
)

func main() {
	dir := "./"
	if len(os.Args) > 1 {
		dir = os.Args[1]
	}
	ips, _ := localIPs()
	fmt.Println("Local IP addresses:")
	for _, v := range ips {
		fmt.Printf("\t%s\n", v)
	}
	fmt.Println("[Static file server] start, port:8080")
	http.Handle("/", http.FileServer(http.Dir(dir)))
	go func() {
		e := http.ListenAndServe(":8080", nil)
		if e != nil {
			os.Exit(1)
		}
	}()
	osCh := make(chan os.Signal, 1)
	fmt.Println("Start Signal Hooker!")
	signal.Notify(osCh, syscall.SIGHUP, syscall.SIGQUIT, syscall.SIGTERM, syscall.SIGINT) // , syscall.SIGSTOP) cannot compile on windows
	fmt.Printf("\rGot a signal [%s]\n", <-osCh)

}

// LocalIPs return all non-loopback IP addresses
func localIPs() ([]string, error) {
	var ips []string
	addrs, err := net.InterfaceAddrs()
	if err != nil {
		return ips, err
	}

	for _, a := range addrs {
		if ipnet, ok := a.(*net.IPNet); ok && !ipnet.IP.IsLoopback() && ipnet.IP.To4() != nil {
			ips = append(ips, ipnet.IP.String())
		}
	}

	return ips, nil
}
