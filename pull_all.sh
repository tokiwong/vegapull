#!/usr/bin/env bash

LANGUAGE="english"
VEGA_DATA=punk-records-data

if [ -d "$VEGA_DATA" ]; then
    read -p "The $VEGA_DATA is about to be wiped to hold the new data, do you want to proceed? (y/N) " confirm
    case $confirm in
        [Yy]* ) ;;
        * ) echo "Aborted"; exit 1 ;;
    esac

    rm -rf $VEGA_DATA
fi

mkdir $VEGA_DATA
echo -e "Created dir: $VEGA_DATA\n"

echo "VegaPulling the list of packs ($LANGUAGE)..."

./target/release/vegapull --language $LANGUAGE packs > $VEGA_DATA/packs.json
COUNT=`jq length $VEGA_DATA/packs.json`

echo -e "Successfully pulled $COUNT packs!\n"

IDX=1
PACKS=`cat $VEGA_DATA/packs.json`
echo "$PACKS" | jq -r '.[].id' | while read id; do
    echo -n "[$IDX/$COUNT] VagaPulling cards for pack '$id'..."
    ./target/release/vegapull --language $LANGUAGE cards $id > "$VEGA_DATA/cards_$id.json"
    echo " OK"
    ((IDX++))
done

echo "Successfully filled the punk records with latest data"
