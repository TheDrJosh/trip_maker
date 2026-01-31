import { drizzle } from "drizzle-orm/node-postgres";

import * as schema from "./schema.ts";

export const db = drizzle(import.meta.env.DATABASE_URL, { schema });
