const fastify = require("fastify")({ logger: false });
const { Pool } = require("pg");

const connectionString = process.env.DB_URL;

// Create a new pool instance
const pool = new Pool({
  connectionString,
});

fastify.get("/", async (request, reply) => {
  return "Hello fastify :3005";
});

fastify.get("/count", async (request, reply) => {
  try {
    const result = await pool.query(
      "SELECT count FROM presence_counters WHERE name = 'group_sittings' ORDER BY updated_at DESC LIMIT 1"
    );
    return reply.send(result.rows[0].count.toString());
  } catch (err) {
    request.log.error(err);
    reply.code(500).send("Internal Server Error");
  }
});

const start = async () => {
  try {
    await fastify.listen({ port: 3005, host: "0.0.0.0" });
    console.log(`Server listening on ${fastify.server.address().port}`);
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();
