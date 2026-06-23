/* This is the implementation presented by Mikkel Thorup in https://arxiv.org/abs/1505.01523
 * It is used as reference implementation for tests.
 */
#include <stdint.h> //defines uintX_t as unsigned X-bit integer.
typedef unsigned __int128 uint128_t;

uint32_t SimpleTab32(uint32_t x, uint32_t H[4][256]) {
  uint32_t i;
  uint32_t h = 0;
  uint8_t c;
  for (i=0; i<4; i++) {
    c = x;
    h ^= H[i][c];
    x = x >> 8;
  }
  return h;
}

uint32_t MixedTab32(uint32_t x, uint64_t H1[4][256], uint32_t H2[4][256]) {
  uint32_t i;
  uint64_t h = 0;
  uint8_t c;
  for (i=0; i<4; i++) {
    c = x;
    h ^= H1[i][c];
    x >>= 8;
  }

  uint32_t derived = h >> 32;
  for (i=0; i<4; i++) {
    c = derived;
    h ^= H2[i][c];
    derived >>= 8;
  }
  return ((uint32_t) h);
}


uint32_t TwistedTab32(uint32_t x, uint64_t H[4][256]) {
  uint32_t i;
  uint64_t h = 0;
  uint8_t c;
  for (i=0; i<3; i++) {
    c = x;
    h ^= H[i][c];
    x = x >> 8;
  }
  c = x^h;
  // extra xor compared with simple
  h ^= H[i][c];
  h >>= 32;
  // extra shift compared with simple
  return ((uint32_t) h);
}





// 64-bit version
uint64_t SimpleTab64(uint64_t x, uint64_t H[8][256]) {
  uint64_t i;
  uint64_t h = 0;
  uint8_t c;
  for (i=0; i<8; i++) {
    c = x;
    h ^= H[i][c];
    x = x >> 8;
  }
  return h;
}

uint64_t MixedTab64(uint64_t x, uint128_t H1[8][256], uint64_t H2[4][256]) {
  uint64_t i;
  uint128_t h = 0;
  uint8_t c;
  for (i=0; i<8; i++) {
    c = x;
    h ^= H1[i][c];
    x >>= 8;
  }

  uint32_t derived = h >> 64;
  for (i=0; i<4; i++) {
    c = derived;
    h ^= H2[i][c];
    derived >>= 8;
  }
  return ((uint64_t) h);
}

/* 64-bit version */
uint64_t TwistedTab64(uint64_t x, uint128_t H[8][256]) {
  uint64_t i;
  uint128_t h = 0;
  uint8_t c;
  for (i=0; i<7; i++) {
    c = x;
    h ^= H[i][c];
    x = x >> 8;
  }
  c = x^h;
  // extra xor compared with simple
  h ^= H[i][c];
  h >>= 64;
  // extra shift compared with simple
  return ((uint64_t) h);
}
