# Zenoh Backend Postgresql
Postgres-based backend plugin for data storage.

The backend assumes a superuser `postgres`  exists with permissions and authorizations within IPv4 context. 
See `pg_conf.hba` for configuration.

The backend creates a `zenoh_data` database.
Deviating from Zenoh's way of doing things, this backend is designed to create a table per topic to optimize queries for each topic and provide an interface that allows us to interact with the database from other applications.

**NOTE**: Topic names may be reutilized and the data can get mixed up. If we can get more metadata information on a sample beyond the topic name we can then construct a hash that embeds said information that represents a collision-safe table name for each node. 

