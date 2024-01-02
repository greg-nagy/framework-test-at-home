import { Elysia } from 'elysia';
import pg from 'pg';

// PostgreSQL connection string
const connectionString = "postgresql://postgres:postgres@database.cdgerttxp3su.eu-central-1.rds.amazonaws.com:5432/portal_dev";

// Create a new PostgreSQL pool
const pool = new pg.Pool({ connectionString });

const app = new Elysia();

app.get('/', async () => 'Hello World');

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

app.listen(3000, () => {
  console.log('Server running on http://localhost:3000');
});

