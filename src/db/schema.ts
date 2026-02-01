import {
    pgTable,
    serial,
    text,
    timestamp,
} from "drizzle-orm/pg-core";



export const users = pgTable("users", {
    id: serial().primaryKey(),
    username: text().notNull(),
    email: text().notNull().unique(),
    createdAt: timestamp("created_at").notNull(),
});

