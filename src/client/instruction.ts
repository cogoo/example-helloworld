import * as BufferLayout from '@solana/buffer-layout';
import {Buffer} from 'buffer';

export function createHelloInstructionData(): Buffer {
  const dataLayout = BufferLayout.struct([
    BufferLayout.u8('instruction')
  ]);

  const data = Buffer.alloc(dataLayout.span);
  dataLayout.encode({
    instruction: 0
  }, data);

  return data;
}

export function createByeInstructionData(): Buffer {
  const dataLayout = BufferLayout.struct([
    BufferLayout.u8('instruction')
  ]);

  const data = Buffer.alloc(dataLayout.span);
  dataLayout.encode({
    instruction: 1
  }, data);

  return data;
}

export function createHelloInstructionDataWithAmount(amount: number): Buffer {
  const dataLayout = BufferLayout.struct(
    [
      BufferLayout.u8('instruction'),
      BufferLayout.u8('amount')
    ]
  );

  console.log({
    dataLayout
  })

  const data = Buffer.alloc(dataLayout.span);
  dataLayout.encode({
    instruction: 2,
    amount
  }, data);

  return data;
}
