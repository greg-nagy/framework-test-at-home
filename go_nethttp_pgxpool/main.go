package main

import (
	"context"
	"fmt"
    "os"
	"log"
	"net/http"

	"github.com/jackc/pgx/v4/pgxpool"
)

type CountResult struct {
    Count int
}

func main() {
    dbConnString := os.Getenv("DB_URL")
    if dbConnString == "" {
        log.Fatal("DB_URL environment variable is not set")
    }

    dbpool, err := pgxpool.Connect(context.Background(), dbConnString)
    if err != nil {
        log.Fatalf("Unable to connect to database: %v\n", err)
    }
    defer dbpool.Close()

    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, "Hello go_nethttp :3007")
    })

    http.HandleFunc("/count", func(w http.ResponseWriter, r *http.Request) {
        var count int
        err := dbpool.QueryRow(context.Background(), "SELECT count FROM presence_counters WHERE name = 'group_sittings' ORDER BY updated_at DESC LIMIT 1").Scan(&count)
        if err != nil {
            http.Error(w, err.Error(), http.StatusInternalServerError)
            return
        }

        fmt.Fprintf(w, "%d", count)
    })

    log.Fatal(http.ListenAndServe(":3007", nil))
}
