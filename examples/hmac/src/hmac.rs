use hacspec_lib::*;
use hacspec_sha256::*;

// HASH_SIZE and K_SIZE are defined in the sha256 crate.
const BLOCK_LEN: usize = K_SIZE;
bytes!(PRK, HASH_SIZE);
bytes!(Block, BLOCK_LEN);

const I_PAD: Block = Block(secret_array!(
    U8,
    [
        0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8,
        0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8,
        0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8,
        0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8,
        0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8,
        0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8, 0x36u8
    ]
));
const O_PAD: Block = Block(secret_array!(
    U8,
    [
        0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8,
        0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8,
        0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8,
        0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8,
        0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8,
        0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8, 0x5cu8
    ]
));

// H(K XOR opad, H(K XOR ipad, text))
pub fn hmac(k: &ByteSeq, txt: &ByteSeq) -> PRK {
    // Applications that use keys longer than B bytes will first hash the key
    // using H and then use the resultant L byte string as the actual key to HMAC
    let mut k_block = Block::new();
    if k.len() > BLOCK_LEN {
        k_block = k_block.update_start(&hash(k));
    } else {
        k_block = k_block.update_start(k);
    }

    let k_ipad = k_block ^ I_PAD;
    let k_opad = k_block ^ O_PAD;

    // TODO: we need something like append in the lib. Or do we want to stick with pre-allocation?
    let mut h_in = ByteSeq::new(BLOCK_LEN + txt.len());
    h_in = h_in.update(0, &k_ipad);
    h_in = h_in.update(BLOCK_LEN, txt);
    let h_inner = hash(&h_in);

    let mut h_in = ByteSeq::new(BLOCK_LEN + h_inner.len());
    h_in = h_in.update(0, &k_opad);
    h_in = h_in.update(BLOCK_LEN, &h_inner);
    PRK::from_seq(&hash(&h_in))
}
