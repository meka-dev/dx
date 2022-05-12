# blocketl

Long-running process extracting block metadata, including information about
transactions. Motivated by the need to collect events for business intelligenc
it should be exhaustive for the possible data that can be collected through
this system. While it is somewhat clear how backfilling would work, it's not
a priority for this iteration. Therefore the system should be considered best
effort.

```
Usage: blocketl [--venue <venue>] [--chain-id <chain-id>] [--params-addr <params-addr>] [--rpc-url <rpc-url>] [--bucket <bucket>] [--service-account <service-account>] [--max-parallel-requests <max-parallel-requests>]

ETL pipeline to extract block and transaction metadata.

Options:
  --venue           comsmos network.
  --chain-id        chain identifier, should be static except on hard fork.
  --params-addr     HTTP server for params reading and manipulation.
                    (127.0.0.1:4445)
  --rpc-url         RPC endpoint serving Tendermint websocket
                    (ws://127.0.0.1:26657/websocket)
  --bucket          GCS bucket for metadata storage
  --service-account path to file containing GCS service account information
  --max-parallel-requests
                    amount of parallel requests to other components
  --help            display usage information
```

Internally the program is structure as a three part pipeline. `Extract` which
receives the latest block and for every transaction in the block calls
`tx_search` to get per transaction metadata (most notable events). `Transform`
which translates the metadata into files and their content. Lastly `Load` which
operates over a stream of files and insert each of them into the storage bucket

## file layout

The following diagram shows the per height layout:

```
/blocketl/
    <hostname>/
        <venue>/
            <chain_id>/
                deadletters/
                    - <height>.json
                    ...
                <height>/
                    - block.json
                    - begin_block.json
                    - end_block.json
                    - events.json
                    txs/
                      - <hash>.json
                      ...
```

## deadletters

Should there be an error at any point in the pipelien a deadletter will be
written to a well known location, with the following content:

``` json
{
  "stage": "Extract",
  "venue": "Terra",
  "height": 7654321,
  "err": "failed to fetch tx info for <hash>"
}
```

