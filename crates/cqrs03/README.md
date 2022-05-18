# CQRS example in ~~the Rails app~~ RTM

**Level: [Advanced](https://github.com/RailsEventStore/ecommerce)**

Migrate using res-ecommerce SQL files and update the entity files after each
migration

The Rails migration scripts are generated as follows:

```bash
for v in $(ls *.rb |cut -d _ -f 1| xargs echo);
do
    rails db:migrate VERSION=$v;
    rails db:schema:dump;
    mv -f ./../structure.sql ${v}_structure.sql;
done
```

Moved to the `migration` folder, entity files are generated for each migration
version:

```bash
export DATABASE_URL="postgres://postgres:secret@localhost:5432/res-ecommerce_development"
for v in {0..38..1};
do
    echo "Migrate to version ${v}"
    pushd migrations
        refinery migrate -e DATABASE_URL -p ./src -t ${v} &>>refinery.log
    popd
    sea-orm-cli generate entity --database-url "${DATABASE_URL}" \
                                --expanded-format \
                                --with-serde both \
                                --include-hidden-tables \
                                --output-dir entities/src &>>entity.log
    git add . &>>add.log
    git commit -m "Evolve[DB]: Migrate to version ${v}" &>>commit.log
done
```
