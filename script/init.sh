export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx database create
# sqlx migrate add create_subscriptions_table

sqlx migrate run
