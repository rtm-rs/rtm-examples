
Introduction: [Sequent](https://www.sequent.io/)

## Simulation

```bash
$ rtm new blog
$ cd blog
$ ls -1
Cargo.toml
all.do
app
blog.rs
config
db
src
tests
```

```bash
cargo install
rtm rtm/db/create
RTM_ENV=test rtm rtm/db/create
rtm rtm/db/create_view_schema
rtm rtm/migrate/online
rtm rtm/migrate/offline
```

If a database exists, you just need to create the `event_store` schema and
the `view_schema`:

```bash
rtm rtm/db/create_event_store
rtm rtm/db/create_view_schema
rtm rtm/migrate/online
rtm rtm/migrate/offline
```

Now we run the specs to ensure we have a working system:

```bash
rtm tests/status
```

## Modelling the domain

```bash
app/           # Non-domain application logic
  projectors/  # Subscribe to events and write to records
  records/     # Ephemeral view tables (RTM/SeaORM/Diesel/ActiveRecord models)
config/        # Configurations to glue everything together
db/            # Database management and configuration
rtm/           # Utility scripts (Rakefile functionality)
src/           # Contains your domain logic
  post/        # Aggregate roots define the namespaces
tests/         # Tests for your application
```

Looking into `src/post/`:

```bash
post/                      # Files are grouped by aggregate root
  commands.rs              # All post command go here
  events.rs                # All post events go here
  post_command_handler.rs  # Subscribes to post commands and dispatches post events
  post.rs                  # The aggregate root
post.rs                    # Requires the entire aggregate root
```
