package main

import (
	"context"
	"fmt"
	"log"
	"net/http"

	"github.com/jackc/pgx/v4/pgxpool"
)

const (
    dbConnectionString = "postgresql://postgres:postgres@database.cdgerttxp3su.eu-central-1.rds.amazonaws.com:5432/portal_dev"
)

type CountResult struct {
    Count int
}

func main() {
    dbpool, err := pgxpool.Connect(context.Background(), dbConnectionString)
    if err != nil {
        log.Fatalf("Unable to connect to database: %v\n", err)
    }
    defer dbpool.Close()

    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, "Hello World")
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

    log.Fatal(http.ListenAndServe(":3000", nil))
}
