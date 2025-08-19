#!/usr/bin/env bash

LANGUAGE="english"
VEGA_DATA=data/$LANGUAGE
VEGA_BIN=target/release/vegapull

if [ -d "$VEGA_DATA" ]; then
    read -rp "The $VEGA_DATA is about to be wiped to hold the new data, do you want to proceed? (y/N) " confirm
    case $confirm in
        [Yy]* ) ;;
        * ) echo "Aborted" >&2; exit 1 ;;
    esac

    rm -rf $VEGA_DATA
fi

mkdir $VEGA_DATA
echo -e "Created dir: $VEGA_DATA\n"

echo "VegaPulling the list of packs ($LANGUAGE)..."

if ! ./target/release/vegapull --language $LANGUAGE packs > $VEGA_DATA/packs.json; then
    echo "Failed to pull list of packs using vegapull. Aborted" >&2
    exit 1
fi

count=$(jq length $VEGA_DATA/packs.json)

echo -e "Successfully pulled $count packs!\n"

function pull_cards() {
    local index=1
    local packs
    packs=$(cat $VEGA_DATA/packs.json)

    while read -r id; do
        echo -n "[$index/$count] VegaPulling cards for pack '$id'..."
        if ! ./target/release/vegapull --language $LANGUAGE cards "$id" > "$VEGA_DATA/cards_$id.json"; then
            echo "Failure"
            echo "Failed to pull cards using vegapull. Aborted" >&2
            return 1
        fi

        echo " OK"
        ((index++))
    done < <( echo "$packs" | jq -r '.[].id')

    echo "Successfully download data for $index packs!"
}

if ! pull_cards; then
    exit 1
fi

function download_images() {
    local index=1
    local packs
    packs=$(cat "$VEGA_DATA/packs.json")

    while read -r pack_id; do
        local output_dir="$VEGA_DATA/images/$pack_id"
        pack_title=$(echo "$packs" | jq --arg pack_id "$pack_id" -r '.[] | select(.id == $pack_id) | .title_parts.title')
        echo "[$index/$count] VegaPulling images for: $pack_title ($pack_id)..."

        if ! "$VEGA_BIN" --language "$LANGUAGE" images --output-dir="$output_dir" "$pack_id" -vv; then
            echo "Failed to pull images using vegapull. Aborted" >&2
            return 1
        else
            echo "[$index/$count] Successfully VegaPulled images for: $pack_title ($pack_id) ✅"
        fi

        ((index++))
    done < <( echo "$packs" | jq -r '.[].id')

    echo "✅ Successfully downloaded data for $index packs!"

}

read -rp "Download card images as well? (y/N) " confirm
case $confirm in
    [Yy]* ) download_images ;;
    * ) ;;
esac

echo "Successfully filled the punk records with latest data"
