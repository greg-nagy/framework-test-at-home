package main

import (
    "context"
    "log"
    "strconv"

    "github.com/gofiber/fiber/v2"
    "github.com/jackc/pgx/v4/pgxpool"
)

func main() {
    // PostgreSQL connection string
    dbConnString := "postgresql://postgres:postgres@database.cdgerttxp3su.eu-central-1.rds.amazonaws.com:5432/portal_dev"

    // Create a new Fiber app
    app := fiber.New()

    // Connect to the database
    dbPool, err := pgxpool.Connect(context.Background(), dbConnString)
    if err != nil {
        log.Fatalf("Unable to connect to database: %v\n", err)
    }
    defer dbPool.Close()

    // Setup routes
    setupRoutes(app, dbPool)

    // Start the server on localhost port 3000
    log.Fatal(app.Listen(":3000"))
}

func setupRoutes(app *fiber.App, dbPool *pgxpool.Pool) {
    // Route for '/'
    app.Get("/", func(c *fiber.Ctx) error {
        return c.SendString("Hello World")
    })

    // Route for '/count'
    app.Get("/count", func(c *fiber.Ctx) error {
        return getCountFromDB(c, dbPool)
    })
}

func getCountFromDB(c *fiber.Ctx, dbPool *pgxpool.Pool) error {
    var count int
    err := dbPool.QueryRow(context.Background(), "SELECT count FROM presence_counters WHERE name = 'group_sittings' ORDER BY updated_at DESC LIMIT 1").Scan(&count)
    if err != nil {
        return c.Status(fiber.StatusInternalServerError).SendString(err.Error())
    }
    return c.SendString(strconv.Itoa(count))
}

