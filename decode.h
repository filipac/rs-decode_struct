#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct SftPriceRet {
  uint64_t amount;
  uint64_t nonce;
  const char *token;
};

struct SftResult {
  bool error;
  const char *error_message;
  const SftPriceRet *item;
};

struct AdvertiseSpaceRet {
  const char *owner;
  uint64_t paid_amount;
  uint64_t paid_until;
  bool is_new;
};

struct SpaceResult {
  bool error;
  const char *error_message;
  const AdvertiseSpaceRet *item;
};

extern "C" {

SftResult *decode_sft_price(const char *base64string);

SpaceResult *decode_advertise_space(const char *base64string);

} // extern "C"
