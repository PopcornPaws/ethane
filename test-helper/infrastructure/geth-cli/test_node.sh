docker pull ethereum/client-go

docker run -it -p 8505:8505 ethereum/client-go --rpc --rpcaddr "127.0.0.1" --rpcport 8505

# Set the default command for the image
# Add a fix private key and 1000 ETH on it.
#CMD ganache-cli -h 0.0.0.0 --fork ${WEB3_FORK_URL} \
#    --account "0x31c354f57fc542eba2c56699286723e94f7bd02a4891a0a7f68566c2a2df6795,100000000000000000000000" \
#    --account "0x31c354f57fc342eba2c56699286723e94f7bd12a4891a0a7f68566c2a2df6795,100000000000000000000000" \
#    --account "0x31c354f57fc342eba2c56699286723e94f7bd12a4891a0a7f68566c2a2d56795,100000000000000000000000"
