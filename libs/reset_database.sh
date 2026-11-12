#!/bin/sh


PREV_DIR=$(pwd)
SCRIPT=$(readlink -f "$0")
DIR=$(dirname "$SCRIPT")

DB="postgres://digivine:digivine@localhost/digivine"


 cd "$DIR/postgres"
 diesel database reset --database-url $DB

#diesel database reset --database-url $DB
for i in "postgres" "terrain"
do
  cd "$DIR/$i"
  diesel migration run --database-url $DB
done
