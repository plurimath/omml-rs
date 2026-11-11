#ifndef OMML_NATIVE_H
#define OMML_NATIVE_H
#include <stddef.h>
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
uint32_t omml_abi_version(void);
/* Borrow input buffers for this call. Own each non-NULL JSON reply and
 * free it once. NULL is a catastrophic allocation/runtime failure. */
char *omml_roundtrip_json_v1(const uint8_t *root, size_t root_len,
                             const uint8_t *xml, size_t xml_len);
void omml_free_json_v1(char *reply);
#ifdef __cplusplus
}
#endif
#endif
