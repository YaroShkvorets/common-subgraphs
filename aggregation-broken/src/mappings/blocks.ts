import {
    ethereum
} from "@graphprotocol/graph-ts"

import {
    Block
} from "../../generated/schema"

export function handleBlock(block: ethereum.Block): void {
    let blockEntity = new Block("auto");
    blockEntity.hash = block.hash;
    blockEntity.number = block.number.toI32();
    blockEntity.timestamp = block.timestamp.toI32();
    blockEntity.save();
}
