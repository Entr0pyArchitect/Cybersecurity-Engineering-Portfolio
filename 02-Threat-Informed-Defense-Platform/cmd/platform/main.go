package main

import (
	"log"
	"net/http"
	"os"

	"threat-informed-defense-platform/internal/server"
)

func main() {
	port := os.Getenv("APP_PORT")
	if port == "" {
		port = "8080"
	}

	app := server.New()

	log.Printf("Threat-Informed Defense Platform starting on port %s", port)
	log.Printf("Scope: mock/public training data only; no real target testing")

	if err := http.ListenAndServe(":"+port, app.Router()); err != nil {
		log.Fatalf("server failed: %v", err)
	}
}
