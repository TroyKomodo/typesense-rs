#/bin/bash

python3 flatten.py typesense-api-spec/openapi.yml openapi.flattened.yml

docker run --rm \
    -u $(id -u):$(id -g) \
    -v $PWD:/local ghcr.io/troykomodo/openapi-generator:rust-gen generate \
    -i /local/openapi.flattened.yml \
    -g rust \
    -o /local/typesense \
    --additional-properties="useSingleRequestParameter=true,packageName=typesense-rs,topLevelApiClient=true,useBonBuilder=true,library=reqwest-trait"

cargo +nightly fmt

git apply generator.patch
