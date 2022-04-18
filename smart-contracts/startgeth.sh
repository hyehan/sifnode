set -e
datadir=/tmp/localgeth
rm -rf $datadir
touch /tmp/geth_password
geth --datadir=$datadir --password /tmp/geth_password account import gethPrivateKey.txt
geth --datadir=$datadir init gethGenesis.json
geth --networkid 9999 --nodiscover --mine --unlock 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --password /tmp/geth_password --allow-insecure-unlock --datadir $datadir --http --http.addr 0.0.0.0 --http.port 8545
