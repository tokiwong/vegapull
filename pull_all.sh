#!/usr/bin/env bash

LANGUAGE="english"
VEGA_DATA=data/$LANGUAGE

if [ -d "$VEGA_DATA" ]; then
    read -p "The $VEGA_DATA is about to be wiped to hold the new data, do you want to proceed? (y/N) " confirm
    case $confirm in
        [Yy]* ) ;;
        * ) echo "Aborted" >&2; exit 1 ;;
    esac

    rm -rf $VEGA_DATA
fi

mkdir $VEGA_DATA
echo -e "Created dir: $VEGA_DATA\n"

echo "VegaPulling the list of packs ($LANGUAGE)..."

./target/release/vegapull --language $LANGUAGE packs > $VEGA_DATA/packs.json
if [[ $? -ne 0 ]]; then
    echo "Failed to pull list of packs using vegapull. Aborted" >&2
    exit 1
fi

COUNT=`jq length $VEGA_DATA/packs.json`

echo -e "Successfully pulled $COUNT packs!\n"

function pull_cards() {
    IDX=1
    PACKS=`cat $VEGA_DATA/packs.json`
    echo "$PACKS" | jq -r '.[].id' | while read id; do
        echo -n "[$IDX/$COUNT] VagaPulling cards for pack '$id'..."
        ./target/release/vegapull -a --language $LANGUAGE cards $id > "$VEGA_DATA/cards_$id.json"
        if [[ $? -ne 0 ]]; then
            echo "Failure"
            echo "Failed to pull cards using vegapull. Aborted" >&2
            return 1
        fi

        echo " OK"
        ((IDX++))
    done

    echo "Successfully download data for $IDX packs!"
}

pull_cards
if [ $? -ne 0 ]; then
    exit 1
fi

function download_images() {
    echo "NOT IMPLEMENTED YET"
}

read -p "Download card images as well? (y/N) " confirm
case $confirm in
    [Yy]* ) download_images ;;
    * ) ;;
esac

echo "Successfully filled the punk records with latest data"
