import { Elysia } from 'elysia';
import pg from 'pg';

// PostgreSQL connection string
const connectionString = process.env.DB_URL;

// Create a new PostgreSQL pool
const pool = new pg.Pool({ connectionString });

const app = new Elysia();

app.get('/', async () => 'Hello bun :3006');

app.get('/count', async () => {
  try {
    const query = 'SELECT count FROM presence_counters WHERE name = \'group_sittings\' ORDER BY updated_at DESC LIMIT 1';
    const { rows } = await pool.query(query);
    return rows[0].count.toString();
  } catch (error) {
    console.error('Database error:', error);
    return { error: 'Internal Server Error' };
  }
});

app.listen({
        port: 3006,
        hostname: '0.0.0.0'
    }, () => {
  console.log('Server running on http://localhost:3006');
});

